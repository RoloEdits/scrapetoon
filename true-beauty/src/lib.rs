use cli_core::ProgressBarFactory;
use core::time;
use line_core::{parse_chapter_list, parse_comments, ChapterListInfo};
use project_core::SeriesConfiguration;
use scraper::Html;
use std::{
    collections::{HashMap, LinkedList},
    thread,
};
use thirtyfour::prelude::*;

pub mod config;
use config::ChapterInfo;

mod story_specific_parsing;

pub async fn parse_chapters(
    start: u16,
    end: u16,
    pages: u16,
    config: &SeriesConfiguration<'_>,
    need_to_skip: fn(u16) -> bool,
) -> LinkedList<ChapterInfo> {
    let chapter_likes_date_map = get_likes_date_hashmap(pages, config.page_url).await;

    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", capabilities)
        .await
        .unwrap();

    let mut result: LinkedList<ChapterInfo> = LinkedList::new();

    let bar = ProgressBarFactory::get_bar(end + 1 - start);

    println!("Parsing Chapters..");

    for chapter in start..=end + config.episode_url_offset {
        if need_to_skip(chapter) {
            continue;
        }

        let url = format!("{}{}", config.episode_url, chapter);

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
                        return result;
                    }
                }
                Ok(ok) => break ok,
            };
        }

        // Needs a delay to wait for everything to load on the page. Go no lower than 3 seconds. Recommened 5.
        // If you notice inconsistent behavior, can increase to see if that solves it.
        thread::sleep(time::Duration::from_secs(5));

        bar.inc(1);

        let html = Html::parse_document(&driver.source().await.unwrap());

        // Works for all stories
        let chapter_number = parse_comments::parse_chapter_number(&html);
        let comment_count = parse_comments::parse_comment_count(&html);
        let date = chapter_likes_date_map
            .get(&chapter_number)
            .unwrap()
            .date
            .to_owned();
        let likes = chapter_likes_date_map.get(&chapter_number).unwrap().likes;
        let comments = parse_comments::parse_user_comments(&html);
        
        result.push_back({
            ChapterInfo {
                chapter_number,
                comment_count,
                likes,
                date,
                comments,
            }
        });
    }

    result
}

struct LikesDate {
    likes: u32,
    date: String,
}

impl LikesDate {
    fn new(likes: u32, date: String) -> Self {
        Self { likes, date }
    }
}

async fn get_likes_date_hashmap(pages: u16, url: &str) -> HashMap<u16, LikesDate> {
    let mut chapter_info_list: LinkedList<ChapterListInfo> = LinkedList::new();
    println!("Pre-Fetching Necessary Data");
    parse_chapter_list::parse_chapter_list_pages(pages, url, &mut chapter_info_list).await;
    println!("Completed Pre-Fetch");

    let mut likes_date_hashmap: HashMap<u16, LikesDate> = HashMap::new();

    for chapter in chapter_info_list {
        match likes_date_hashmap.insert(
            chapter.chapter_number,
            LikesDate::new(chapter.likes, chapter.date),
        ) {
            None => continue,
            Some(_) => continue,
        };
    }

    likes_date_hashmap
}
