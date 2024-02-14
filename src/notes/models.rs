use std::collections::HashMap;

use chrono::{DateTime, Utc};
use iced::widget::text_editor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Data {
    pub categories: HashMap<Uuid, Category>,
    pub posts: HashMap<Uuid, Post>,
}

#[derive(Debug)]
pub struct NotesState {
    pub data: Data,
    pub selected_post_id: Option<Uuid>,
    pub text_editor_state: text_editor::Content,
    pub search_input_value: String,
    pub is_loading: bool,
    pub is_dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


impl NotesState {
    pub fn get_ordered_posts_list(&self) -> Vec<&Post> {
        let mut posts_list: Vec<&Post> = self.data.posts.iter().map(|(_id, post)| post).collect();
        posts_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        posts_list
    }
    pub fn set_selected_post(&mut self, post_id: Option<Uuid>) {
        self.selected_post_id = post_id;
        self.text_editor_state = text_editor::Content::with_text(&self.get_selected_post().unwrap().content)
    }
    pub fn add_post(&mut self, title: &str, content: &str) -> Post {
        let new_post = Post::new(title, content);
        self.data.posts.insert(new_post.id, new_post.clone());
        new_post
    }

    pub fn remove_post(&mut self, post_id: Uuid) -> Option<Post> {
        self.data.posts.remove(&post_id)
    }

    pub fn update_post(&mut self, updated_post: Post) -> Option<Post> {
        self.data.posts.insert(updated_post.id, updated_post)
    }

    pub fn get_selected_post(&self) -> Option<&Post> {
        match self.selected_post_id {
            Some(post_id) => self.data.posts.get(&post_id),
            None => None
        }
    }
    pub fn update_selected_post_title(&mut self, next_title: String) {
        match self.get_selected_post() {
            Some(post) => {
                self.update_post(Post {
                    title: next_title,
                    ..post.clone()
                })
            }
            None => {
                panic!("Failed to update selected post title")
            }
        };
    }
    pub fn update_selected_post_content(&mut self, next_content: String) {
        match self.get_selected_post() {
            Some(post) => {
                self.update_post(Post {
                    content: next_content,
                    ..post.clone()
                });
            }
            None => {
                panic!("Failed to update selected post content")
            }
        };
    }
}


impl Post {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::from(title),
            content: String::from(content),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn is_selected(&self, selected_post_id: &Option<Uuid>) -> bool {
        match selected_post_id {
            Some(selected_post_id) => { self.id == selected_post_id.clone() }
            None => { false }
        }
    }
}


impl Default for NotesState {
    fn default() -> Self {
        Self {
            data: Default::default(),
            selected_post_id: None,
            is_loading: false,
            is_dirty: false,
            text_editor_state: iced::widget::text_editor::Content::with_text(""),
            search_input_value: String::from(""),
        }
    }
}