use std::collections::LinkedList;

use line_core::UserComment;
use project_core::SeriesConfiguration;

pub struct ChapterInfo {
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
    filename: "unordinary",

    page_url: "https://www.webtoons.com/en/super-hero/unordinary/list?title_no=679",

    episode_url: "https://www.webtoons.com/en/super-hero/unordinary/episode-285/viewer?title_no=679&episode_no=",

    episode_url_offset: 5,
};

type Skip = fn(u16) -> bool;

pub const TO_SKIP: Skip = |chapter: u16| -> bool {
    matches!(
        chapter,
        1 | 37 | 48 | 80 | 81 | 82 | 162 | 163 | 164 | 165 | 166 | 246 | 247
    )
};
