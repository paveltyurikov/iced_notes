use iced::{Element, font, Font, Length, theme, widget};

use crate::notes::{buttons, common, FontSize, Message, models, post_details};

fn panel_post_list_item(post: &models::Post, is_selected: bool) -> Element<Message> {
    let btn = widget::button(
        widget::Column::new()
            .push(
                widget::text(&post.title).size(14).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::default()
                })
            ).width((FontSize::Body as i32 * 12) as f32)
            .push(
                common::post_date_created(post).size(10)
            )
    )
        .on_press(Message::SelectPost(post.id));
    if is_selected {
        btn.style(theme::Button::Positive).into()
    } else {
        btn.style(theme::Button::Text).into()
    }
}

fn panel_post_list_empty<'a>() -> Element<'a, Message> {
    widget::Column::new().spacing(8)
        .width(Length::Fixed((FontSize::Body as i32 * 12) as f32))
        .push(widget::Text::new(String::from("No posts found")))
        .into()
}


pub fn panel_posts_list<'a>(state: &'a models::NotesState) -> Element<'a, Message> {
    if state.data.posts.len() > 0 {
        let posts: Element<Message> = state.get_ordered_posts_list()
            .into_iter()
            .fold(
                widget::Column::new().spacing(8),
                |col, post| {
                    col.push(
                        panel_post_list_item(
                            &post,
                            post.is_selected(&state.selected_post_id),
                        ))
                }).into();
        widget::container(widget::column(vec![
            buttons::IconButtonVariant::button_post_add(),
            widget::scrollable(
                widget::row(vec![
                    widget::horizontal_space(4).into(),
                    widget::column(vec![posts]).into(),
                    widget::horizontal_space(14).into(),
                ])
            ).into(),
        ]))


            .into()
    } else {
        widget::container(widget::column(vec![
            buttons::IconButtonVariant::button_post_add(),
            widget::scrollable(
                widget::row(vec![
                    widget::horizontal_space(4).into(),
                    panel_post_list_empty().into(),
                    widget::horizontal_space(14).into(),
                ])
            ).into(),
        ]))


            .into()
    }
}


pub fn panel_post_selected<'a>(state: &'a models::NotesState) -> Element<'a, Message> {
    match state.get_selected_post() {
        Some(post) => {
            post_details::details_view(state, post).into()
        }
        None => {
            post_details::details_view_empty()
        }
    }
}

