use super::chapter::models::Chapter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Story {
    pub story_page: StoryPage,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryPage {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub status: String,
    pub release_day: String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
}

pub struct LikesDate {
    pub likes: u32,
    pub date: String,
}

impl LikesDate {
    pub const fn new(likes: u32, date: String) -> Self {
        Self { likes, date }
    }
}
