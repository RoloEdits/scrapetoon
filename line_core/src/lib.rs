pub mod chapter_height_pixels;
pub mod chapter_list;
pub mod comments;
pub mod daily_schedule;
pub mod panels;
pub mod series_info;

use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::VecDeque;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ChapterListInfo {
    pub chapter: u16,
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

#[derive(Debug, Serialize)]
pub struct SeriesInfo {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub status: String,
    pub release_day: String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
    pub chapter_list_info: VecDeque<ChapterListInfo>,
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

#[derive(Serialize, Debug)]
pub struct UserComment {
    pub user: Option<String>,
    pub contents: Option<String>,
    pub post_date: Option<String>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub reply_count: Option<u16>,
}

impl UserComment {
    #[must_use]
    pub const fn new(
        user: Option<String>,
        body: Option<String>,
        post_date: Option<String>,
        upvotes: Option<u32>,
        downvotes: Option<u32>,
        reply_count: Option<u16>,
    ) -> Self {
        Self {
            user,
            contents: body,
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

pub trait CommentSum {
    fn sum_total_comments(&self) -> u32;
}

#[derive(Serialize, Debug)]
pub struct GenericChapterInfo {
    pub chapter_number: u16,
    pub comments: u32,
    pub likes: u32,
    pub date: String,
    pub user_comments: VecDeque<UserComment>,
    pub chapter_length: u32,
    pub skips_adjusted_count: u16,
}

impl CommentSum for VecDeque<GenericChapterInfo> {
    fn sum_total_comments(&self) -> u32 {
        let mut accumulator = 0;
        for chapter in self {
            accumulator += chapter.comments;
        }

        accumulator
    }
}

// TODO: Implement for all stories using a Options for those fields that arent generic

#[derive(Serialize, Debug)]
pub struct GenericRecord<'a> {
    pub title: &'a String,
    pub author: &'a String,
    pub genre: &'a String,
    pub status: &'a String,
    pub release_day: &'a String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
    pub season: Option<u8>,
    pub season_chapter: Option<u16>,
    pub arc: &'a Option<String>,
    pub chapter: u16,
    pub length: u32,
    pub comments: u32,
    // TODO: Add total_replies once that is implemented.
    pub total_comments: u32,
    pub likes: u32,
    pub total_likes: u32,
    pub published: &'a String,
    pub user: Option<String>,
    pub comment_body: Option<String>,
    pub post_date: Option<String>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub replies: Option<u16>,
    pub scrape_date: &'a String,
}

impl GenericRecord<'_> {
    #![allow(unused)]
    fn write(
        path: &Path,
        chapter_info: VecDeque<GenericChapterInfo>,
        series_info: SeriesInfo,
        filename: &str,
    ) -> Result<()> {
        let csv_name = format!("{filename}.csv");
        let mut writer = csv::Writer::from_path(path.join(csv_name)).unwrap();

        let total_likes = series_info.sum_total_likes();
        let total_comments = chapter_info.sum_total_comments();

        let title = series_info.title;
        let author = series_info.author;
        let genre = series_info.genre;
        let status = series_info.status;
        let release_day = series_info.release_day;
        let views = series_info.views;
        let subscribers = series_info.subscribers;
        let rating = series_info.rating;

        for chap in chapter_info {
            let chapter = chap.skips_adjusted_count;
            let comments = chap.comments;
            let likes = chap.likes;
            let published = chap.date;
            let length = chap.chapter_length;
            let season = None;
            let season_chapter = None;
            let arc = None;
            let scrape_date = project_core::get_current_utc_date();

            for comment in chap.user_comments {
                let user = comment.user;
                let comment_body = comment.contents;
                let post_date = comment.post_date;
                let upvotes = comment.upvotes;
                let downvotes = comment.downvotes;
                let replies = comment.reply_count;

                let record_data = GenericRecord {
                    title: &title,
                    author: &author,
                    genre: &genre,
                    status: &status,
                    release_day: &release_day,
                    views,
                    subscribers,
                    rating,
                    season,
                    season_chapter,
                    arc: &arc,
                    chapter,
                    length,
                    comments,
                    total_comments,
                    likes,
                    total_likes,
                    published: &published,
                    user,
                    comment_body,
                    post_date,
                    upvotes,
                    downvotes,
                    replies,
                    scrape_date: &scrape_date,
                };

                writer
                    .serialize(record_data)
                    .context("Couldn't write to file.")?;
            }
        }

        writer.flush().expect("Couldn't flush buffer.");

        Ok(())
    }
}
