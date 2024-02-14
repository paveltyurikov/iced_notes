use iced::{Alignment, alignment, Element, Length, widget};
use iced::widget::{row, Text};

use crate::notes::{DEFAULT_DATE_FORMAT, Message};
use crate::notes::models::Post;

pub fn post_date_created(post: &Post) -> Text {
    Text::new(format!("{}", post.created_at.format(DEFAULT_DATE_FORMAT)))
}


pub fn controls_row(content: Vec<Element<Message>>) -> Element<Message> {
    widget::Container::new(
        row(content)
            .spacing(4)
            .padding(4)
            .align_items(Alignment::Center)
    )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right)
        .into()
}