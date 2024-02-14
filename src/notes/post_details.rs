use iced::{alignment, Element, Length, widget};
use iced::widget::text_editor;

use crate::notes::{buttons, common, FontSize, Message, models};

pub fn details_view<'a>(state: &'a models::NotesState, selected_post: &'a models::Post) -> Element<'a, Message> {
    let title = widget::text_input("Enter Title", &selected_post.title)
        .on_input(Message::PostTitleUpdated)
        .size(FontSize::Header1);

    let created_at = common::post_date_created(selected_post)
        .width(Length::Fill)
        .size(FontSize::Caption)
        .horizontal_alignment(alignment::Horizontal::Left);


    let content = text_editor(&state.text_editor_state)
        .on_action(Message::OnTextEditorAction);

    let post_component = widget::Column::new()
        .spacing(16)
        .push(common::controls_row(vec![
            buttons::IconButtonVariant::button_post_delete(selected_post).into()
        ]))
        .push(title)
        .push(created_at)
        .push(content);

    widget::Container::new(post_component)
        .max_width(800)
        .width(Length::Fill)
        .padding(16)
        .center_x()
        .into()
}

pub fn details_view_empty<'a>() -> Element<'a, Message> {
    let title = widget::Text::new("No title")
        .width(Length::Fill)
        .size(FontSize::Header1)
        .horizontal_alignment(alignment::Horizontal::Left);
    let created_at = widget::Text::new("No date")
        .width(Length::Fill)
        .size(FontSize::Caption)
        .horizontal_alignment(alignment::Horizontal::Left);
    let head = iced::widget::column(vec![title.into(), created_at.into()]);
    let content = widget::Text::new("No selected post")
        .width(Length::Fill)
        .size(FontSize::Body)
        .horizontal_alignment(alignment::Horizontal::Left);
    let post_component = widget::Column::new()
        .spacing(16)
        .push(head)
        .push(content);
    iced::widget::row(vec![
        widget::horizontal_space(Length::Fixed(32.0))
            .into(),
        widget::Container::new(post_component)
            .max_width(800)
            .width(Length::Fill)
            .padding(16)
            .center_x()
            .into(),
    ])
        .into()
}