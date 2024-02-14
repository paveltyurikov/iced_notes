use iced::{alignment, Border, Color, Element, theme, Theme, Vector, widget};

use crate::notes::{FontSize, Message, models};
use crate::notes::buttons;
use crate::notes::icons::{Icons, material_icon};

#[repr(i32)]
#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    Small = 24,
    #[default]
    Normal = 36,
    Large = 48,
}

pub enum IconButtonVariant {
    Outlined(ButtonSize),
    Filled(ButtonSize),
    Text(ButtonSize),
}


impl From<ButtonSize> for f32 {
    fn from(value: ButtonSize) -> Self {
        value as i32 as f32
    }
}


impl From<IconButtonVariant> for iced::theme::Button {
    fn from(value: IconButtonVariant) -> Self {
        theme::Button::Custom(Box::new(value))
    }
}


impl IconButtonVariant {
    fn get_button_size(&self) -> &ButtonSize {
        match self {
            Self::Filled(size) => size,
            Self::Outlined(size) => size,
            Self::Text(size) => size
        }
    }
    fn get_icon_size(&self, button_size: &ButtonSize) -> f32 {
        match button_size {
            ButtonSize::Small => ButtonSize::Small as i32 as f32 * 0.4,
            ButtonSize::Normal => ButtonSize::Normal as i32 as f32 * 0.5,
            ButtonSize::Large => ButtonSize::Large as i32 as f32 * 0.6
        }
    }


    fn render_button<'a, Message: 'a>(self, icon: Icons, on_press: Option<Message>) -> widget::Button<'a, Message> {
        let button_size = self.get_button_size();
        let icon_size = self.get_icon_size(button_size);
        let btn = widget::button(
            widget::container(
                material_icon(icon)
                    .size(icon_size)
            )
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
        )
            .width(*button_size as i32 as f32)
            .height(*button_size as i32 as f32)
            .style(self);
        if let Some(on_press) = on_press {
            return btn.on_press(on_press);
        }
        return btn;
    }
    pub fn render_button_with_tooltip<'a, Message: 'a + std::clone::Clone>(btn: widget::Button<'a, Message>, label: &'a str) -> Element<'a, Message> {
        widget::tooltip(
            btn,
            label,
            widget::tooltip::Position::FollowCursor,
        )
            .style(theme::Container::Box)
            .size(FontSize::Caption)
            .into()
    }
    pub fn button_post_add<'a>() -> Element<'a, Message> {
        icon_button(
            Icons::PostAdd,
            "Create post",
            ButtonSize::default(),
            Some(Message::ButtonCreatePressed),
        ).into()
    }

    pub fn button_post_delete(post: &models::Post) -> Element<Message> {
        icon_button(
            Icons::DeleteForever,
            "Remove post",
            ButtonSize::default(),
            Some(Message::ButtonDeletePressed(post.id)),
        )
            .into()
    }
    pub fn theme_button<'a>(is_theme_dark: &bool, button_size: buttons::ButtonSize) -> Element<'a, Message> {
        icon_button(match is_theme_dark {
            true => Icons::LightMode,
            false => Icons::DarkMode,
        },
                    "Toggle theme",
                    button_size,
                    Some(
                        Message::ToggleIsDarkMode(
                            !is_theme_dark.clone()
                        )
                    ),
        )
    }
}


pub fn icon_button<'a, Message: Clone + 'a>(
    icon: Icons,
    label: &'a str,
    size: ButtonSize,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let btn = IconButtonVariant::Outlined(size)
        .render_button(icon, on_press);
    IconButtonVariant::render_button_with_tooltip(btn, label)
}

impl widget::button::StyleSheet for IconButtonVariant {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> widget::button::Appearance {
        let palette = style.extended_palette();
        let border_color = match self {
            IconButtonVariant::Filled(_) => palette.primary.base.color,
            IconButtonVariant::Outlined(_) => style.palette().primary,
            _ => Color::TRANSPARENT,
        };
        let background_color = match self {
            IconButtonVariant::Filled(_) => {
                palette.primary.strong.color
            }
            _ => Color::TRANSPARENT,
        };


        let border_radius = match self {
            Self::Outlined(size) => {
                let res: f32 = (*size).into();
                res / 2.0
            }
            Self::Filled(size) => {
                let res: f32 = (*size).into();
                res / 2.0
            }
            _ => 0.0
        };

        let text_color = match self {
            Self::Filled(_) => palette.primary.strong.text,
            _ => palette.primary.strong.color
        };

        widget::button::Appearance {
            text_color,
            background: Some(background_color.into()),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: border_radius.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> widget::button::Appearance {
        let active = self.active(style);
        let palette = style.extended_palette();
        let text_color = match self {
            Self::Filled(_) => palette.primary.base.text,
            _ => palette.primary.base.color
        };
        let background_color = match self {
            IconButtonVariant::Filled(_) => {
                palette.primary.base.color
            }
            _ => Color::TRANSPARENT,
        };
        widget::button::Appearance {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            text_color,
            background: Some(background_color.into()),
            ..active
        }
    }
}