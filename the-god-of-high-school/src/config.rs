use std::collections::LinkedList;

use line_core::UserComment;
use project_core::SeriesConfiguration;

pub struct ChapterInfo {
    pub chapter_number: u16,
    pub comments: u32,
    pub likes: u32,
    pub date: String,
    pub user_comments: LinkedList<UserComment>,
}

pub trait CommentSum {
    fn sum_total_comments(&self) -> u32;
}

impl CommentSum for LinkedList<ChapterInfo> {
    fn sum_total_comments(&self) -> u32 {
        let mut accumulator = 0;
        for chapter in self {
            accumulator += chapter.comments;
        }

        accumulator
    }
}

pub const CONFIG: SeriesConfiguration = SeriesConfiguration {

    filename: "the-god-of-high-school",

    page_url: "https://www.webtoons.com/en/action/the-god-of-high-school/list?title_no=66",

    episode_url: "https://www.webtoons.com/en/action/the-god-of-high-school/ep-1/viewer?title_no=66&episode_no=",

    episode_url_offset: 1,
};

type Skip = fn(u16) -> bool;

pub const TO_SKIP: Skip = |chapter: u16| -> bool { matches!(chapter, 249) };
