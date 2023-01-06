use std::collections::LinkedList;
use std::path::Path;
use line_core::SeriesInfo;
use lore_olympus::config::{ChapterInfo, CommentSum};

pub fn write(
    path: &Path,
    chapter_info: &LinkedList<ChapterInfo>,
    series_info: &SeriesInfo,
    filename: &str,
) {
    let csv_name = format!("{filename}.csv");
    let mut writer = csv::Writer::from_path(path.join(csv_name)).unwrap();

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
            "chapter_length",
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
            "season",
        ])
        .expect("Couldn't write to file.");

    let title = series_info.title.clone();
    let author = series_info.author.clone();
    let genre = series_info.genre.clone();
    let status = series_info.status.clone();
    let release_day = series_info.release_day.clone();
    let views = series_info.views.to_string();
    let subscribers = series_info.subscribers.to_string();
    let rating = series_info.rating.to_string();
    let total_comments = chapter_info.sum_total_comments().to_string();
    let total_likes = series_info.sum_total_likes().to_string();

    for chapter in chapter_info {
        let season = chapter.season.to_string();
        let meaningful_chapter_number = chapter.meaningful_chapter_number.to_string();

        // let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comments.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.clone();
        let chapter_length = chapter.chapter_length.to_string();
        let current_utc_date = project_core::get_current_utc_date();

        for comment in &chapter.user_comments {
            let user = comment.user.clone();
            let comment_body = comment.body.clone();
            let post_date = comment.post_date.clone();
            let upvotes = comment.upvotes.to_string();
            let downvotes = comment.downvotes.to_string();
            let reply_count = comment.reply_count.to_string();

            writer
                .write_record([
                    &title,
                    &author,
                    &genre,
                    &status,
                    &release_day,
                    &views,
                    &subscribers,
                    &rating,
                    &meaningful_chapter_number,
                    &chapter_length,
                    &comments,
                    &total_comments,
                    &likes,
                    &total_likes,
                    &date,
                    &user,
                    &comment_body,
                    &post_date,
                    &upvotes,
                    &downvotes,
                    &reply_count,
                    &current_utc_date,
                    &season,
                ])
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
