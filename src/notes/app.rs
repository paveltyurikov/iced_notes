use iced::{Application, Command, Element, Length, Theme, widget};
use iced::widget::{column, container};

use crate::notes::{buttons, common, Message, models, post_list, type_to_async};
use crate::notes::storage::JsonStorage;

#[derive(Debug, Default)]
pub struct IcedApplication {
    pub notes: models::NotesState,
    pub is_dark_theme: bool,
}


impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            IcedApplication::default(),
            Command::perform(JsonStorage::load(), Message::NotesStorageLoaded),
        )
    }


    fn title(&self) -> String {
        String::from("notes")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NotesStorageLoaded(data) => {
                match data {
                    Ok(data) => {
                        self.notes.data = data;
                    }
                    Err(error) => {
                        print!("Message::JsonStorageLoaded {:?}", error);
                    }
                };
                Command::none()
            }
            Message::ToggleIsDarkMode(value) => {
                self.is_dark_theme = value;
                Command::none()
            }
            Message::ButtonDeletePressed(post_id) => {
                self.notes.remove_post(post_id);
                Command::perform(
                    JsonStorage::save::<models::Data>(self.notes.data.clone()),
                    Message::NotesStorageSaved,
                )
            }
            Message::ButtonCreatePressed => {
                let new_post = self.notes.add_post("New", "");
                self.notes.selected_post_id = Some(new_post.id);
                Command::perform(
                    JsonStorage::save::<models::Data>(self.notes.data.clone()),
                    Message::NotesStorageSaved,
                )
            }
            Message::SelectPost(post_id) => {
                self.notes.set_selected_post(Some(post_id));
                Command::none()
            }
            Message::PostTitleUpdated(next_title) => {
                self.notes.update_selected_post_title(next_title);
                Command::perform(
                    JsonStorage::save::<models::Data>(self.notes.data.clone()),
                    Message::NotesStorageSaved,
                )
            }
            Message::OnTextEditorAction(action) => {
                self.notes.is_dirty = self.notes.is_dirty || action.is_edit();

                self.notes.text_editor_state.perform(action);

                Command::perform(
                    type_to_async(
                        Message::PostContentUpdated(self.notes.text_editor_state.text())
                    ),
                    |text| text,
                )
            }
            Message::PostContentUpdated(text) => {
                self.notes.update_selected_post_content(text);
                Command::perform(
                    JsonStorage::save::<models::Data>(self.notes.data.clone()),
                    Message::NotesStorageSaved,
                )
            }
            _ => {
                Command::perform(
                    JsonStorage::save::<models::Data>(self.notes.data.clone()),
                    Message::NotesStorageSaved,
                )
            }
        }
    }


    fn theme(&self) -> Self::Theme {
        match self.is_dark_theme {
            true => Theme::Dark,
            false => Theme::Light
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column(vec![
                common::controls_row(
                    vec![
                        buttons::IconButtonVariant::theme_button(
                            &self.is_dark_theme, buttons::ButtonSize::Small,
                        ),
                    ])
                    .into(),
                widget::row(vec![
                    post_list::panel_posts_list(&self.notes),
                    post_list::panel_post_selected(&self.notes),
                ])
                    .into(),
            ])
                .width(Length::Fill)
        )
            .width(Length::Fill)
            .into()
    }
}




