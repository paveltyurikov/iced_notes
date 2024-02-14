use std::path::PathBuf;

use iced::Pixels;
use iced::widget::text_editor;
use uuid::Uuid;

use crate::notes::storage::StorageError;

pub mod app;
pub mod buttons;
pub mod common;
pub mod constants;
pub mod icons;
pub mod models;
pub mod post_list;

pub mod post_details;
pub mod storage;
pub mod styles;


pub static DEFAULT_DATE_FORMAT: &str = "%H:%M %d.%m.%Y";


#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontSize {
    Body = 16,
    Caption = 10,
    Header1 = 24,
    Tooltip = 12,
}

impl From<FontSize> for Pixels {
    fn from(value: FontSize) -> Self {
        let value: i32 = value as i32;
        Pixels(value as f32)
    }
}

#[warn(dead_code)]
fn get_storage_file() -> PathBuf {
    PathBuf::from("../../notes_data.json")
}

pub async fn type_to_async<T>(t: T) -> T {
    t
}

#[derive(Debug, Clone)]
pub enum Message {
    NotesStorageLoaded(Result<models::Data, StorageError>),
    NotesStorageSaved(Result<(), StorageError>),
    SelectPost(Uuid),
    ToggleIsDarkMode(bool),
    ButtonCreatePressed,
    ButtonDeletePressed(Uuid),
    PostTitleUpdated(String),
    PostContentUpdated(String),
    OnTextEditorAction(text_editor::Action),
}