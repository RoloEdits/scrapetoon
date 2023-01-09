use cli_core::ProgressBarFactory;
use core::time;
use line_core::{chapter_height_pixels, comments, SeriesInfo};
use project_core::SeriesConfiguration;
use scraper::Html;
use std::{collections::LinkedList, thread};
use thirtyfour::prelude::*;

pub mod config;
use config::ChapterInfo;

mod story_specific_parsing;

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
        .unwrap();

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

        // Exponential back-off
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
                        // If fails to connect it will return any already scraping
                        eprintln!("Error connecting to webpage, saving progress and exiting...");
                        return (series_info, result);
                    }
                }
                Ok(ok) => break ok,
            };
        }

        // Needs a delay to wait for everything to load on the page. Go no lower than 3 seconds. Recommend 5.
        // If you notice inconsistent behavior, can increase to see if that solves it.
        thread::sleep(time::Duration::from_secs(5));

        let html = Html::parse_document(&driver.source().await.unwrap());

        // Story specific
        let meaningful_chapter_number =
            story_specific_parsing::parse_meaningful_chapter_number(&html);

        // Works for all stories
        let chapter_number = chapter - skips;
        let comments = comments::parse_comment_count(&html);
        let date = chapter_likes_date_map
            .get(&chapter_number)
            .unwrap()
            .date
            .clone();

        let likes = chapter_likes_date_map.get(&chapter_number).unwrap().likes;
        let user_comments = comments::parse_users(&html);
        let chapter_length = chapter_height_pixels::from(&html);

        result.push_back({
            ChapterInfo {
                meaningful_chapter_number,

                chapter_number,
                comments,
                likes,
                date,
                user_comments,
                chapter_length,
            }
        });
    }

    driver.quit().await.unwrap();

    (series_info, result)
}
