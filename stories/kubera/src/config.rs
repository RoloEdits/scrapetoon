use std::collections::VecDeque;

use line_core::UserComment;
use project_core::SeriesConfiguration;

// Need to change data accordingly.
pub struct ChapterInfo {
    // Might need to remove or tweak.
    pub season: u8,
    pub season_chapter: u16,
    pub arc_title: String,
    // Everything below will work for all stories.
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

    filename: "kubera",

    page_url: "https://www.webtoons.com/en/fantasy/kubera/list?title_no=83",

    episode_url: "https://www.webtoons.com/en/fantasy/kubera/season-3-ep-264-abyss-23/viewer?title_no=83&episode_no=",

    episode_url_offset: 0,
};

type Skip = fn(u16) -> bool;

pub const TO_SKIP: Skip = |chapter: u16| -> bool { matches!(chapter, |102| 267 | 284 | 285) };
