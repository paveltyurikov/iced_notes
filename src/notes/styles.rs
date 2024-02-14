use iced::{Border, Color, Shadow, Theme};

#[derive(Default)]
pub enum ButtonVariant {
    #[default]
    Container,
    Small,
    Medium,
    Large,
}


impl From<ButtonVariant> for iced::theme::Button {
    fn from(value: ButtonVariant) -> Self {
        iced::theme::Button::Custom(Box::new(value))
    }
}

impl iced::widget::button::StyleSheet for ButtonVariant {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let background_color = match self {
            ButtonVariant::Container => style.palette().text,
            _ => style.palette().text,
        };


        let border_width = match self {
            ButtonVariant::Container => 1.0,
            _ => 1.0
        };

        iced::widget::button::Appearance {
            shadow: Shadow {
                color: Color::from_rgb(0.0, 0.0, 255.0),
                ..Shadow::default()
            },
            text_color: style.palette().background,
            background: Some(background_color.into()),
            border: Border {
                color: style.palette().text,
                width: border_width,
                radius: 12.0.into(),
            },
            ..Default::default()
        }
    }
}

// impl RowBtn {
//     pub fn new(selected: bool) -> Self {
//         if selected {
//             Self::selected()
//         } else {
//             Self::default()
//         }
//     }
//
//     pub fn new_bordered(selected: bool) -> Self {
//         if selected {
//             Self::selected()
//         } else {
//             Self::lightly_bordered()
//         }
//     }
//
//     pub fn selected() -> Self {
//         Self(RowButtonVariant::Selected)
//     }
//
//     pub fn lightly_bordered() -> Self {
//         Self(RowButtonVariant::LightlyBordered)
//     }
// }
//
// impl From<RowBtn> for iced::theme::Button {
//     fn from(value: RowBtn) -> Self {
//         iced::theme::Button::Custom(Box::new(value))
//     }
// }
//
// impl iced::widget::button::StyleSheet for RowBtn {
//     type Style = Theme;
//
//     fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
//         let background_color = match self.0 {
//             RowButtonVariant::Default => iced::Color::TRANSPARENT,
//             RowButtonVariant::LightlyBordered => iced::Color::TRANSPARENT,
//             RowButtonVariant::Selected => style.extended_palette().secondary.base.color,
//         };
//
//         let border_color = match self.0 {
//             RowButtonVariant::Default => iced::Color::TRANSPARENT,
//             RowButtonVariant::LightlyBordered => iced::Color {
//                 a: 0.1,
//                 ..style.palette().text
//             },
//             RowButtonVariant::Selected => style.extended_palette().secondary.weak.color,
//         };
//
//         let border_width = match self.0 {
//             RowButtonVariant::Default => 0.0,
//             RowButtonVariant::LightlyBordered => 1.0,
//             RowButtonVariant::Selected => 1.0,
//         };
//
//         iced::widget::button::Appearance {
//             text_color: style.palette().text,
//             background: Some(background_color.into()),
//             border: Border::with_radius(6.0),
//             ..Default::default()
//         }
//     }
// }



