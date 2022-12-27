use cli_core::ProgressBarFactory;
use core::time;
use line_core::{parse_chapter_list, parse_comments, ChapterInfo, UserComment};
use scraper::Html;
use std::{
    collections::{HashMap, LinkedList},
    thread,
};
use thirtyfour::prelude::*;

mod parse;

pub struct TowerOfGodChapterInfo {
    pub season: u8,
    pub season_chapter: u16,
    pub chapter_number: u16,
    pub comment_count: u32,
    pub likes: u32,
    pub date: String,
    pub comments: LinkedList<UserComment>,
}

pub async fn parse_chapters(start: u16, end: u16, pages: u16) -> LinkedList<TowerOfGodChapterInfo> {
    let chapter_likes_date_map = get_likes_date_hashmap(pages).await;

    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", capabilities)
        .await
        .unwrap();

    let mut result: LinkedList<TowerOfGodChapterInfo> = LinkedList::new();

    let bar = ProgressBarFactory::get_bar(end + 1 - start);

    println!("Parsing Chapters..");

    for chapter in start..=end + 1 {
        // The URl no=221 for chapter 221 is a 404. No=222 is where #221 is.
        if chapter == 221 {
            continue;
        }

        let url = format!("https://www.webtoons.com/en/fantasy/tower-of-god/season-1-ep-0/viewer?title_no=95&episode_no={}", chapter);

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

        let chapter_number = parse_comments::parse_chapter_number(&html);
        let date = chapter_likes_date_map
            .get(&chapter_number)
            .unwrap()
            .date
            .to_owned();
        let likes = chapter_likes_date_map.get(&chapter_number).unwrap().likes;

        result.push_back(TowerOfGodChapterInfo {
            season: parse::parse_season_number(&html),
            season_chapter: parse::parse_season_chapter_number(&html),
            chapter_number,
            comment_count: parse_comments::parse_comment_count(&html),
            date,
            likes,
            comments: parse_comments::parse_user_comments(&html),
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

async fn get_likes_date_hashmap(pages: u16) -> HashMap<u16, LikesDate> {
    const CHAPTER_LIST_URL: &str =
        r"https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95";
    let mut chapter_info_list: LinkedList<ChapterInfo> = LinkedList::new();
    println!("Pre-Fetching Necessary Data");
    parse_chapter_list::parse_chapter_list_pages(pages, CHAPTER_LIST_URL, &mut chapter_info_list)
        .await;
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
