use std::collections::LinkedList;

use true_beauty::config::ChapterInfo;

pub fn write(path: &str, chapter_info: &LinkedList<ChapterInfo>, filename: &str) {
    let final_path = format!("{}{}.csv", path, filename);
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    writer
        // The resulting data columns. Tweak as needed.
        .write_record([
            // Works for all stories
            "chapter",
            "comments",
            "likes",
            "date",
            "user",
            "comment_body",
            "post_date",
            "upvotes",
            "downvotes",
            "reply_count",
            "scrape_date",
        ])
        .expect("Couldn't write to file.");

    for chapter in chapter_info {
        
        // These functions work over all stories
        let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comment_count.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.to_owned();

        let current_utc_date = project_core::get_current_utc_date();

        for comment in &chapter.comments {
            let user = comment.user.to_owned();
            let comment_body = comment.body.to_owned();
            let post_date = comment.post_date.to_owned();
            let upvotes = comment.upvotes.to_string();
            let downsvotes = comment.downvotes.to_string();
            let reply_count = comment.reply_count.to_string();

            writer
                // These just need to match the previously given columns.
                .write_record([
                    &chapter_number,
                    &comments,
                    &likes,
                    &date,
                    &user,
                    &comment_body,
                    &post_date,
                    &upvotes,
                    &downsvotes,
                    &current_utc_date,
                    &reply_count,
                ])
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
