use iced::widget::Text;

use crate::notes::constants::MATERIAL_ICONS_FONT;

const UNKNOWN: char = '\u{eabd}';

pub enum Icons {
    DarkMode,
    LightMode,
    DeleteForever,
    PostAdd,
}

pub fn get_codepoint(icon: &Icons) -> char {
    match icon {
        Icons::DarkMode => '\u{e51c}',
        Icons::LightMode => '\u{e518}',
        Icons::DeleteForever => '\u{e92b}',
        Icons::PostAdd => '\u{ea20}',
    }
}

pub fn material_icon<'a>(icon: Icons) -> Text<'a> {
    Text::new(String::from(get_codepoint(&icon)))
        .font(MATERIAL_ICONS_FONT)
}

