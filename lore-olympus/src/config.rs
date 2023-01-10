use std::collections::LinkedList;

use line_core::UserComment;
use project_core::SeriesConfiguration;

pub struct ChapterInfo {
    pub season: u8,
    pub meaningful_chapter_number: u16,

    pub chapter_number: u16,
    pub comments: u32,
    pub likes: u32,
    pub date: String,
    pub user_comments: LinkedList<UserComment>,
    pub chapter_length: u32,
    pub skips_adjusted_count: u16,
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
    filename: "lore-olympus",

    page_url: "https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320",

    episode_url: "https://www.webtoons.com/en/romance/lore-olympus/episode-1/viewer?title_no=1320&episode_no=",

    episode_url_offset: 0,
};

type Skip = fn(u16) -> bool;

// #13: Message and Concept Art
// #18: Q&A
// #31: Hiatus special short
// #47: QnA
// #120: Season 1 Recap
pub const TO_SKIP: Skip = |chapter: u16| -> bool { matches!(chapter, 13 | 18 | 31 | 47 | 120) };
