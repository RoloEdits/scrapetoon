use std::collections::LinkedList;

use line_core::UserComment;
use project_core::SeriesConfiguration;

pub struct ChapterInfo {
    pub chapter_number: u16,
    pub comment_count: u32,
    pub likes: u32,
    pub date: String,
    pub comments: LinkedList<UserComment>,
}

pub const CONFIG: SeriesConfiguration = SeriesConfiguration {

    filename: "true-beauty",

    page_url: "https://www.webtoons.com/en/romance/truebeauty/list?title_no=1436",

    episode_url: "https://www.webtoons.com/en/romance/truebeauty/episode-0-/viewer?title_no=1436&episode_no=",

    episode_url_offset: 0,
};

type Skip = fn(u16) -> bool;

pub const TO_SKIP: Skip = |_chapter: u16| -> bool {

    false

    // match chapter {
        
    //     _ => false,
    // }
};
