pub mod parse;
mod regex;

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
    pub release_day: String,
}
// There is duplication but doing to keeps implimenting simpler and less cluttered
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
