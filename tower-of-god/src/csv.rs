use std::collections::LinkedList;

use tower_of_god::TowerOfGodChapterInfo;

pub fn write(path: &str, tog_chapter_info: &LinkedList<TowerOfGodChapterInfo>) {
    let final_path = format!("{}{}", path, "tower-of-god.csv");
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    writer
        .write_record(["season", "season_chapter", "chapter", "comments", "likes", "date", "user", "comment_body", "post_date", "upvotes", "downvotes", "reply_count", "scrape_date"])
        .expect("Couldn't write to file.");

    for chapter in tog_chapter_info {
        let season = chapter.season.to_string();
        let season_chapter = chapter.season_chapter.to_string();
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
                .write_record(&[&season, &season_chapter, &chapter_number, &comments, &likes, &date, &user, &comment_body, &post_date, &upvotes, &downsvotes, &reply_count, &current_utc_date])
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}