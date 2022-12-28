use std::collections::LinkedList;

use line_core::SeriesInfo;
use tower_of_god::config::{ChapterInfo, CommentSum};

pub fn write(
    path: &str,
    chapter_info: &LinkedList<ChapterInfo>,
    series_info: &SeriesInfo,
    filename: &str,
) {
    let final_path = format!("{}{}.csv", path, filename);
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    writer
        // The resulting data columns. Tweak as needed.
        .write_record([
            // Works for all stories
            "title",
            "author",
            "genre",
            "status",
            "release_day",
            "views",
            "subscribers",
            "rating",
            "chapter",
            "comments",
            "total_comments",
            "likes",
            "total_likes",
            "date",
            "user",
            "comment_body",
            "post_date",
            "upvotes",
            "downvotes",
            "reply_count",
            "scrape_date",
            // Story specific
            "season",
            "season_chapter",
        ])
        .expect("Couldn't write to file.");

    let title = series_info.title.to_owned();
    let author = series_info.author.to_owned();
    let genre = series_info.genre.to_owned();
    let status = series_info.status.to_owned();
    let release_day = series_info.release_day.to_owned();
    let views = series_info.views.to_string();
    let subscribers = series_info.subscribers.to_string();
    let rating = series_info.rating.to_string();
    let total_comments = chapter_info.sum_total_comments().to_string();
    let total_likes = series_info.sum_total_likes().to_string();

    for chapter in chapter_info {
        // Might need to change or delete these depending on the story
        let season = chapter.season.to_string();
        let season_chapter = chapter.season_chapter.to_string();

        // These functions work over all stories
        let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comments.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.to_owned();
        let current_utc_date = project_core::get_current_utc_date();

        for comment in &chapter.user_comments {
            let user = comment.user.to_owned();
            let comment_body = comment.body.to_owned();
            let post_date = comment.post_date.to_owned();
            let upvotes = comment.upvotes.to_string();
            let downsvotes = comment.downvotes.to_string();
            let reply_count = comment.reply_count.to_string();

            writer
                // These just need to match the previously given columns.
                .write_record([
                    &title,
                    &author,
                    &genre,
                    &status,
                    &release_day,
                    &views,
                    &subscribers,
                    &rating,
                    &chapter_number,
                    &comments,
                    &total_comments,
                    &likes,
                    &total_likes,
                    &date,
                    &user,
                    &comment_body,
                    &post_date,
                    &upvotes,
                    &downsvotes,
                    &reply_count,
                    &current_utc_date,
                    // story specific
                    &season,
                    &season_chapter,
                ])
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
