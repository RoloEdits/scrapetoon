use std::collections::VecDeque;

use line_core::UserComment;
use project_core::SeriesConfiguration;

pub struct ChapterInfo {
    pub chapter_number: u16,
    pub comments: u32,
    pub likes: u32,
    pub date: String,
    pub user_comments: VecDeque<UserComment>,
    pub chapter_length: u32,
    pub skips_adjusted_count: u16,
}

pub trait CommentSum {
    fn sum_total_comments(&self) -> u32;
}

impl CommentSum for VecDeque<ChapterInfo> {
    fn sum_total_comments(&self) -> u32 {
        let mut accumulator = 0;
        for chapter in self {
            accumulator += chapter.comments;
        }

        accumulator
    }
}

pub const CONFIG: SeriesConfiguration = SeriesConfiguration {
    filename: "true-beauty",

    page_url: "https://www.webtoons.com/en/romance/truebeauty/list?title_no=1436",

    episode_url:
        "https://www.webtoons.com/en/romance/truebeauty/episode-0-/viewer?title_no=1436&episode_no=",

    episode_url_offset: 0,
};

type Skip = fn(u16) -> bool;

pub const TO_SKIP: Skip = |_chapter: u16| -> bool { false };
