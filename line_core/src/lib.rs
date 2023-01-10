pub mod chapter_height_pixels;
pub mod chapter_list;
pub mod comments;
pub mod daily_schedule;
pub mod panels;
pub mod series_info;

use std::collections::LinkedList;

#[derive(Debug)]
pub struct ChapterListInfo {
    pub chapter_number: u16,
    pub likes: u32,
    pub date: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DailyScheduleInfo {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub total_likes: u32,
    pub status: String,
    pub day: String,
}

#[derive(Debug)]
pub struct SeriesInfo {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub status: String,
    pub release_day: String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
    pub chapter_list_info: LinkedList<ChapterListInfo>,
}
impl SeriesInfo {
    #[must_use]
    pub fn sum_total_likes(&self) -> u32 {
        let mut accumulator = 0;
        for chapter in &self.chapter_list_info {
            accumulator += chapter.likes;
        }

        accumulator
    }
}
pub struct UserComment {
    pub user: String,
    pub body: String,
    pub post_date: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub reply_count: u16,
}

impl UserComment {
    #[must_use]
    pub const fn new(
        user: String,
        body: String,
        post_date: String,
        upvotes: u32,
        downvotes: u32,
        reply_count: u16,
    ) -> Self {
        Self {
            user,
            body,
            post_date,
            upvotes,
            downvotes,
            reply_count,
        }
    }
}

pub struct LikesDate {
    pub likes: u32,
    pub date: String,
}

impl LikesDate {
    #[must_use]
    pub const fn new(likes: u32, date: String) -> Self {
        Self { likes, date }
    }
}
