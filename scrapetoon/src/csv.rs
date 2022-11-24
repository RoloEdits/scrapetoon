use hashlink::LinkedHashSet;

use line_core::{DailyScheduleInfo, SeriesInfo};

pub fn write_daily_schedule(path: &str, daily_schedule: &LinkedHashSet<DailyScheduleInfo>) {
    let final_path = format!("{}{}", path, "daily_schedule.csv");
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    writer
        .write_record(&["title", "author", "genre", "total_likes", "status"])
        .expect("Couldn't write to file.");

    for story in daily_schedule {
        let title = format!("{}", story.title);
        let author = format!("{}", story.author);
        let genre = format!("{}", story.genre);
        let total_likes = format!("{}", story.total_likes);
        let status = format!("{}", story.status);

        writer
            .write_record(&[title, author, genre, total_likes, status])
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}

pub fn write_series_info(path: &str, series_info: &SeriesInfo) {
    let cleaned_filename = series_info
        .title
        .replace("/", "")
        .replace("<", "")
        .replace(">", "")
        .replace(":", "")
        .replace(r#"""#, "")
        .replace(r"\", "")
        .replace("|", "")
        .replace("?", "")
        .replace("*", "");

    let final_path = format!("{}{}", path, format!("{}.csv", cleaned_filename));
    let mut writer =
        csv::Writer::from_path(final_path).expect("The system cannot find the path specified");

    writer
        .write_record(&[
            "title",
            "author",
            "genre",
            "total_likes",
            "status",
            "release_day",
            "views",
            "subscribers",
            "rating",
            "chapter_number",
            "likes",
            "date",
        ])
        .expect("Couldn't write to file.");

    for chapter in series_info.chapter_info_list.iter() {
        let title = format!("{}", series_info.title);
        let author = format!("{}", series_info.author);
        let genre = format!("{}", series_info.genre);
        let total_likes = format!("{}", series_info.sum_total_likes());
        let status = format!("{}", series_info.status);
        let release_day = format!("{}", series_info.release_day);
        let views = format!("{}", series_info.views);
        let subscribers = format!("{}", series_info.subscribers);
        let rating = format!("{}", series_info.rating);

        let chapter_number = format!("{}", chapter.chapter_number);
        let likes = format!("{}", chapter.likes);
        let date = format!("{}", chapter.date);

        writer
            .write_record(&[
                title,
                author,
                genre,
                total_likes,
                status,
                release_day,
                views,
                subscribers,
                rating,
                chapter_number,
                likes,
                date,
            ])
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}
