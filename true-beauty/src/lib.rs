use cli_core::ProgressBarFactory;
use core::time;
use line_core::{chapter_height_pixels, comments, SeriesInfo};
use project_core::SeriesConfiguration;
use scraper::Html;
use std::{collections::LinkedList, thread};
use thirtyfour::prelude::*;

pub mod config;
use config::ChapterInfo;

/// # Panics
///
/// Will panic if `ChromeDriver` isn't running
pub async fn parse_chapters(
    start: u16,
    end: u16,
    pages: u16,
    config: &SeriesConfiguration<'_>,
    need_to_skip: fn(u16) -> bool,
) -> (SeriesInfo, LinkedList<ChapterInfo>) {
    let (series_info, chapter_likes_date_map) =
        line_core::series_info::get_extra_info(pages, config.page_url).await;

    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", capabilities)
        .await
        .expect("ChromeDriver not running.");

    let mut result: LinkedList<ChapterInfo> = LinkedList::new();

    let bar = ProgressBarFactory::get_bar(end + 1 - start);

    println!("Parsing Chapters..");

    let mut skips = 0;
    for chapter in start..=end + config.episode_url_offset {
        bar.inc(1);

        if need_to_skip(chapter) {
            skips += 1;
            continue;
        }

        let url = format!("{}{chapter}", config.episode_url);

        let mut retries = 5;
        let mut wait = 1;
        loop {
            match driver.goto(&url).await {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait));
                        wait *= 2;
                    } else {
                        eprintln!("Error connecting to webpage, saving progress and exiting...");
                        return (series_info, result);
                    }
                }
                Ok(ok) => break ok,
            };
        }

        thread::sleep(time::Duration::from_secs(5));

        let html = Html::parse_document(&driver.source().await.unwrap());

        let chapter_number = chapter - skips;
        let comment_count = comments::parse_comment_count(&html);
        let date = chapter_likes_date_map
            .get(&chapter_number)
            .unwrap()
            .date
            .clone();
        let likes = chapter_likes_date_map.get(&chapter_number).unwrap().likes;
        let comments = comments::parse_users(&html);
        let chapter_length = chapter_height_pixels::from(&html);

        result.push_back({
            ChapterInfo {
                chapter_number,
                comments: comment_count,
                likes,
                date,
                user_comments: comments,
                chapter_length,
            }
        });
    }

    driver.quit().await.unwrap();

    (series_info, result)
}
