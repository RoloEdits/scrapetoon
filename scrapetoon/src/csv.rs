use static_assertions::assert_eq_size_val;
use std::collections::LinkedList;
use std::path::Path;

use line_core::{DailyScheduleInfo, SeriesInfo};

pub fn write_daily_schedule(path: &Path, daily_schedule: &LinkedList<DailyScheduleInfo>) {
    let final_path = path.join("daily_schedule.csv");
    let mut writer = csv::Writer::from_path(final_path).unwrap();

    let header = [
        "title",
        "author",
        "genre",
        "day",
        "total_likes",
        "status",
        "scrape_date",
    ];

    writer
        .write_record(header)
        .expect("Couldn't write to file.");

    for story in daily_schedule {
        let title = story.title.as_str();
        let author = story.author.as_str();
        let genre = story.genre.as_str();
        let day = story.day.as_str();
        let total_likes = format!("{}", story.total_likes);
        let status = story.status.as_str();
        let current_utc_date = project_core::get_current_utc_date();

        let record_data = [
            title,
            author,
            genre,
            day,
            &total_likes,
            status,
            &current_utc_date,
        ];

        // Sanity check to make sure both header and record_data match in length
        assert_eq_size_val!(header, record_data);

        writer
            .write_record(record_data)
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}

pub fn write_series_info(path: &Path, series_info: &SeriesInfo) {
    let cleaned_filename = series_info
        .title
        .replace(['/', '<', '>', ':', '"', '\\', '|', '?', '*'], "");

    // let filename = format!("{cleaned_filename}.csv");
    let final_path = path.join(cleaned_filename).with_extension("csv");
    // let final_path = format!("{path}{filename}");
    let mut writer =
        csv::Writer::from_path(final_path).expect("The system cannot find the path specified");

    let header = [
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
        "chapter",
        "likes",
        "chapter_release_date",
        "scrape_date",
    ];

    writer
        .write_record(header)
        .expect("Couldn't write to file.");

    for chapter in &series_info.chapter_list_info {
        let title = series_info.title.as_str();
        let author = series_info.author.as_str();
        let genre = series_info.genre.as_str();
        let total_likes = series_info.sum_total_likes().to_string();
        let status = series_info.status.as_str();
        let release_day = series_info.release_day.as_str();
        let views = series_info.views.to_string();
        let subscribers = series_info.subscribers.to_string();
        let rating = series_info.rating.to_string();
        let total_chapters = series_info.chapter_list_info.len().to_string();

        let chapter_number = chapter.chapter_number.to_string();
        let likes = chapter.likes.to_string();
        let chapter_release_date = chapter.date.as_str();

        let current_utc_date = project_core::get_current_utc_date();

        let record_data = [
            title,
            author,
            genre,
            &total_likes,
            status,
            release_day,
            &views,
            &subscribers,
            &rating,
            &total_chapters,
            &chapter_number,
            &likes,
            chapter_release_date,
            &current_utc_date,
        ];

        // Sanity check to make sure both header and record_data match in length
        assert_eq_size_val!(header, record_data);

        writer
            .write_record(record_data)
            .expect("Couldn't write to file.");
    }

    writer.flush().expect("Couldn't flush buffer.");
}
