pub mod parse_chapter_list;
pub mod parse_comments;
pub mod parse_daily_schedule;
pub mod parse_series_info;

use project_core::regex;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct ChapterInfo {
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
    pub chapter_info_list: LinkedList<ChapterInfo>,
}

impl SeriesInfo {
    pub fn sum_total_likes(&self) -> u32 {
        let mut accumulator = 0;
        for chapter in self.chapter_info_list.iter() {
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
    pub fn new(
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
