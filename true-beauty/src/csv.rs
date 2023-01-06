use line_core::SeriesInfo;
use static_assertions::assert_eq_size_val;
use std::collections::LinkedList;
use std::path::Path;
use true_beauty::config::{ChapterInfo, CommentSum};

pub fn write(
    path: &Path,
    chapter_info: &LinkedList<ChapterInfo>,
    series_info: &SeriesInfo,
    filename: &str,
) {
    let csv_name = format!("{filename}.csv");
    let mut writer = csv::Writer::from_path(path.join(csv_name)).unwrap();

    let header = [
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
    ];

    writer
        .write_record(header)
        .expect("Couldn't write to file.");

    let title = series_info.title.as_str();
    let author = series_info.author.as_str();
    let genre = series_info.genre.as_str();
    let status = series_info.status.as_str();
    let release_day = series_info.release_day.as_str();
    let views = series_info.views.to_string();
    let subscribers = series_info.subscribers.to_string();
    let rating = series_info.rating.to_string();
    let total_comments = chapter_info.sum_total_comments().to_string();
    let total_likes = series_info.sum_total_likes().to_string();

    for chapter in chapter_info {
        let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comments.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.as_str();
        let chapter_length = chapter.chapter_length.to_string();

        let current_utc_date = project_core::get_current_utc_date();

        for comment in &chapter.user_comments {
            let user = comment.user.as_str();
            let comment_body = comment.body.as_str();
            let post_date = comment.post_date.as_str();
            let upvotes = comment.upvotes.to_string();
            let downvotes = comment.downvotes.to_string();
            let reply_count = comment.reply_count.to_string();

            let record_data = [
                title,
                author,
                genre,
                status,
                release_day,
                &views,
                &subscribers,
                &rating,
                &chapter_number,
                &chapter_length,
                &comments,
                &total_comments,
                &likes,
                &total_likes,
                date,
                user,
                comment_body,
                post_date,
                &upvotes,
                &downvotes,
                &reply_count,
                &current_utc_date,
            ];

            // Sanity check to make sure both header and record_data match in length
            assert_eq_size_val!(header, record_data);

            writer
                .write_record(record_data)
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
