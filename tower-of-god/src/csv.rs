use std::collections::LinkedList;
use std::path::Path;
use line_core::SeriesInfo;
use tower_of_god::config::{ChapterInfo, CommentSum};

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
            // Story specific
            "season",
            "season_chapter",
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
        // Might need to change or delete these depending on the story
        let season = chapter.season.to_string();
        let season_chapter = chapter.season_chapter.to_string();

        // These functions work over all stories
        let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comments.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.clone();
        let current_utc_date = project_core::get_current_utc_date();
        let chapter_length = chapter.chapter_length.to_string();

        for comment in &chapter.user_comments {
            let user = comment.user.clone();
            let comment_body = comment.body.clone();
            let post_date = comment.post_date.clone();
            let upvotes = comment.upvotes.to_string();
            let downvotes = comment.downvotes.to_string();
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
                    // story specific
                    &season,
                    &season_chapter,
                ])
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
