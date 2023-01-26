

// use anyhow::{bail, Context, Result};
// use cli_core::ProgressBarFactory;
// use core::time;
// use line_core::{chapter_height_pixels, LikesDate};
// use line_core::{comments, SeriesInfo};
// use project_core::SeriesConfiguration;
// use scraper::Html;
// use std::collections::{HashMap, VecDeque};
// use std::thread;
// use thirtyfour::prelude::*;
// 
// pub mod config;
// use config::ChapterInfo;
// 
// pub fn parse_chapters(
//     start: u16,
//     end: u16,
//     pages: u16,
//     config: &SeriesConfiguration<'_>,
//     need_to_skip: fn(u16) -> bool,
// ) -> Result<(SeriesInfo, VecDeque<ChapterInfo>)> {
//     let (series_info, chapter_likes_date_map) =
//         line_core::series_info::get_extra_info(pages, config.page_url)?;
// 
//     let result = work(start, end, config, need_to_skip, &chapter_likes_date_map)?;
// 
//     Ok((series_info, result))
// }
// 
// #[tokio::main]
// async fn work(
//     start: u16,
//     end: u16,
//     config: &SeriesConfiguration<'_>,
//     need_to_skip: fn(u16) -> bool,
//     chapter_likes_date_map: &HashMap<u16, LikesDate>,
// ) -> Result<VecDeque<ChapterInfo>> {
//     let capabilities = DesiredCapabilities::chrome();
//     let driver = WebDriver::new("http://localhost:9515", capabilities)
//         .await
//         .context("ChromeDriver not running")?;
// 
//     let mut result: VecDeque<ChapterInfo> = VecDeque::new();
// 
//     let bar = ProgressBarFactory::get_bar(end + 1 - start);
// 
//     println!("Parsing Chapters..");
// 
//     let mut skips = 0;
//     for chapter in start..=end + config.episode_url_offset {
//         bar.inc(1);
// 
//         if need_to_skip(chapter) {
//             skips += 1;
//             continue;
//         }
// 
//         let url = format!("{}{chapter}", config.episode_url);
// 
//         // Exponential back-off
//         let mut retries = 5;
//         let mut wait = 1;
//         loop {
//             match driver.goto(&url).await {
//                 Err(_) => {
//                     if retries > 0 {
//                         retries -= 1;
//                         thread::sleep(time::Duration::from_secs(wait));
//                         wait *= 2;
//                     } else {
//                         bail!("Failed connecting to webpage, saving progress and exiting...");
//                     }
//                 }
//                 Ok(ok) => break ok,
//             };
//         }
// 
//         // Needs a delay to wait for everything to load on the page. Go no lower than 3 seconds. Recommend 5.
//         // If you notice inconsistent behavior, can increase to see if that solves it.
//         thread::sleep(time::Duration::from_secs(5));
// 
//         let html = Html::parse_document(
//             &driver
//                 .source()
//                 .await
//                 .context("Failed to get source from driver response")?,
//         );
// 
//         let skips_adjusted_count = chapter - skips;
//         let chapter_number = comments::parse_chapter_number(&html)?;
//         let comment_count = comments::parse_comment_count(&html)?;
//         let date = chapter_likes_date_map
//             .get(&chapter_number)
//             .unwrap()
//             .date
//             .clone();
//         let likes = chapter_likes_date_map
//             .get(&chapter_number)
//             .expect("Failed to get likes and date data")
//             .likes;
//         let comments = comments::parse_users(&html)?;
//         let chapter_length = chapter_height_pixels::from(&html)?;
// 
//         result.push_back({
//             ChapterInfo {
//                 chapter_number,
//                 comments: comment_count,
//                 likes,
//                 date,
//                 user_comments: comments,
//                 chapter_length,
//                 skips_adjusted_count,
//             }
//         });
//     }
// 
//     driver.quit().await.unwrap();
// 
//     Ok(result)
// }
