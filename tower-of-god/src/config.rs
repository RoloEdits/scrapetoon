use std::collections::LinkedList;

use line_core::UserComment;
use project_core::SeriesConfiguration;

// Need to change data accordingly.
pub struct ChapterInfo {
    // Might need to remove or tweak.
    pub season: u8,
    pub season_chapter: u16,
    // Everything below will work for all stories.
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
    // eg. 'tower-of-god.csv' is the output
    filename: "tower-of-god",

    // Make sure to only put up to the 'title_no=' and then whatever the ID is.
    // Remove the &page={}' if needed.  
    page_url: "https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95",

    // Make sure that the URL ends with the equals. The numbers after point to the specific episode page but we dont need that.
    episode_url: "https://www.webtoons.com/en/fantasy/tower-of-god/season-3-ep-133/viewer?title_no=95&episode_no=",

    // Where you would put the URL 'episode_no=' offset.
    // If the chapter number and the URL number dont match, put the difference here.
    // eg. episode 550 of ToG is really 551 in the URL number, so you put 1 here.
    // This is due to a URL no mishap, specificly no=221.
    // The goal is to align the chapter number seen on the list page with the input values used in the CLI run.
    // This makes it much easier to use for everyone and gives expected results.
    episode_url_offset: 1,
};

type Skip = fn(u16) -> bool;

// If there is any need to skip a number, this is where you put it.
// As a rule of thumb, if you need to put an offset, there will be a the same number of skips as the offset.
// The page response of a bad episode URL will be a 404
// When running this will crash the program.
pub const TO_SKIP: Skip = |chapter: u16| -> bool {
    // Add the url number that you need to skip here and return true.
    // For first test run this should always return false. There may not be a need to skip anything.
    match chapter {
        // The URl no=221 for chapter 221 is a 404. No=222 is where #221 is.
        221 => true,
        _ => false,
    }
};
