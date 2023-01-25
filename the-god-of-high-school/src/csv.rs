use anyhow::{Context, Result};
use line_core::{GenericRecord, SeriesInfo};

use std::collections::VecDeque;
use std::path::Path;
use the_god_of_high_school::config::{ChapterInfo, CommentSum};

pub fn write(
    path: &Path,
    chapter_info: VecDeque<ChapterInfo>,
    series_info: SeriesInfo,
    filename: &str,
) -> Result<()> {
    let csv_name = format!("{filename}.csv");
    let mut writer = csv::Writer::from_path(path.join(csv_name)).unwrap();

    let total_comments = chapter_info.sum_total_comments();
    let total_likes = series_info.sum_total_likes();

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
