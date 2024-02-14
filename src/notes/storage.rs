use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::notes::{models::Data, get_storage_file};

#[derive(Debug, Clone)]
pub enum StorageError {
    SaveFileError,
    LoadFileError,
    SaveFileFormatError,
    LoadFileFormatError,
    WriteError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonStorage {
    path: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl JsonStorage {
    pub async fn load() -> Result<Data, StorageError> {
        use async_std::prelude::*;
        let mut contents = String::new();
        let mut file = async_std::fs::File::open(get_storage_file())
            .await
            .map_err(|_| StorageError::LoadFileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| StorageError::LoadFileError)?;

        serde_json::from_str(&contents)
            .map_err(|_| StorageError::LoadFileFormatError)
    }

    pub async fn save<'a, T: Serialize>(notes: T) -> Result<(), StorageError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&notes)
            .map_err(|_| {
                StorageError::SaveFileFormatError
            })?;

        let path = get_storage_file();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| StorageError::SaveFileError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| StorageError::SaveFileError)?;

            file.write_all(json.as_bytes())
                .await
        }.map_err(|_| StorageError::WriteError)?;

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl JsonStorage {
    fn storage() -> Option<web_sys::JsonStorage> {
        let window = web_sys::window()?;

        window.local_storage().ok()?
    }

    async fn load() -> Result<JsonStorage, LoadError> {
        let storage = Self::storage().ok_or(LoadError::FileError)?;

        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::FileError)?
            .ok_or(LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::FileError)?;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;

        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::WriteError)?;

        let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}