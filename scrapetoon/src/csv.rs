use hashlink::LinkedHashSet;

use line_core::{DailyScheduleInfo, SeriesInfo};

pub fn write_daily_schedule(path: &str, daily_schedule: &LinkedHashSet<DailyScheduleInfo>) {
    let final_path = format!("{}{}", path, "daily_schedule.csv");
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    writer
        .write_record(["title", "author", "genre", "total_likes", "status", "scrape_date"])
        .expect("Couldn't write to file.");

    for story in daily_schedule {
        let title = story.title.to_string();
        let author = story.author.to_string();
        let genre = story.genre.to_string();
        let total_likes = format!("{}", story.total_likes);
        let status = story.status.to_string();
        let current_utc_date = project_core::get_current_utc_date();

        writer
        .write_record(&[title, author, genre, total_likes, status, current_utc_date])
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}

pub fn write_series_info(path: &str, series_info: &SeriesInfo) {
    let cleaned_filename = series_info
        .title
        .replace(['/', '<', '>', ':', '"', '\\', '|', '?', '*'], "");

    let filename = format!("{}.csv", cleaned_filename);
    let final_path = format!("{}{}", path, filename);
    let mut writer =
        csv::Writer::from_path(final_path).expect("The system cannot find the path specified");

    writer
        .write_record([
            "title",
            "author",
            "genre",
            "total_likes",
            "status",
            "release_day",
            "views",
            "subscribers",
            "rating",
            "total_chapters",
            "chapter_number",
            "likes",
            "chater_release_date",
            "scrape_date"
        ])
        .expect("Couldn't write to file.");

    for chapter in series_info.chapter_info_list.iter() {
        let title = series_info.title.to_string();
        let author = series_info.author.to_string();
        let genre = series_info.genre.to_string();
        let total_likes =  series_info.sum_total_likes().to_string();
        let status = series_info.status.to_string();
        let release_day = series_info.release_day.to_string();
        let views = series_info.views.to_string();
        let subscribers = series_info.subscribers.to_string();
        let rating =series_info.rating.to_string();
        let total_chapters = series_info.chapter_info_list.len().to_string();

        let chapter_number =  chapter.chapter_number.to_string();
        let likes = chapter.likes.to_string();
        let chapter_release_date = chapter.date.to_string();

        let current_utc_date = project_core::get_current_utc_date();

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
                total_chapters,
                chapter_number,
                likes,
                chapter_release_date,
                current_utc_date
            ])
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}
