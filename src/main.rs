use iced::{Application, Settings};

mod notes;

pub fn main() -> iced::Result {
    notes::app::IcedApplication::run(Settings {
        fonts: vec![
            include_bytes!("../fonts/material-icons.ttf")
                .as_slice()
                .into(),
        ],
        ..Settings::default()
    })
}




