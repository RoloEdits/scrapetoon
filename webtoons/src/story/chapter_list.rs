#![allow(unused)]

use crate::factories::BlockingReferClientFactory;
use anyhow::{anyhow, bail, Context, Result};
use chrono::NaiveDate;
use crossbeam::queue::SegQueue;
use indicatif::ParallelProgressIterator;
use models::ChapterList;
use rand::prelude::*;
use rayon::prelude::*;
use scraper::{ElementRef, Html, Selector};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

pub mod models;

pub fn parse(end: u16, input_url: &str) -> Result<VecDeque<ChapterList>> {
    // 8 Threads is around the line at which problems start to occur when pinging out too many times at once as all getting blocked
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .context("Couldn't create thread pool")?;

    let range: Vec<_> = (1..=end).collect();
    let total = range.len() as u64;

    let chapter_info: SegQueue<ChapterList> = SegQueue::new();

    range
        .into_par_iter()
        .progress_count(total)
        .try_for_each(|page| {
            let url = format!("{input_url}&page={page}");
            if list(&url, &chapter_info).is_err() {
                // TODO: Log
                bail!("Failed to parse Page {page}")
            }
            Ok(())
        })?;

    let mut result: VecDeque<ChapterList> = VecDeque::with_capacity(chapter_info.len());

    for info in chapter_info {
        result.push_back(info);
    }

    Ok(result)
}

fn list(url: &str, chapter_info: &SegQueue<ChapterList>) -> Result<()> {
    let response = BlockingReferClientFactory::get(url)?;
    let mut rng = thread_rng();
    let rand = rng.gen_range(1..=3);
    thread::sleep(Duration::from_millis(500 * rand));

    let html = response
        .text()
        .with_context(|| format!("Failed to text body result info at url: {url}"))?;

    chapter(&html, chapter_info)?;

    Ok(())
}

fn chapter(html: &str, chapter_info: &SegQueue<ChapterList>) -> Result<()> {
    let html = Html::parse_document(html);
    let chapter_selector =
        Selector::parse("ul#_listUl>li").expect("Failed to parse Chapter Selector");

    for chapter in html.select(&chapter_selector) {
        let chapter_number = number(&chapter)?;
        let likes = likes(&chapter)?;
        let date = date(&chapter)?;
        chapter_info.push(ChapterList {
            chapter: chapter_number,
            likes,
            date,
        });
    }

    Ok(())
}

// TODO: Combine combine all implementations to use either ElementRef or HTML
fn number(html: &ElementRef) -> Result<u16> {
    let chapter_number_selector =
        Selector::parse("span.tx").expect("Failed to parse Chapter Number Selector");

    let chapter_number = html
        .select(&chapter_number_selector)
        .next()
        .ok_or_else(|| anyhow!("No chapter number to parse"))?
        .text()
        .collect::<Vec<_>>();

    let cleaned = chapter_number
        .first()
        .ok_or_else(|| anyhow!("No chapter number to parse"))?
        .replace('#', "");

    let result = cleaned
        .parse::<u16>()
        .with_context(|| format!("Failed to parse {cleaned} into a u16"))?;

    Ok(result)
}

fn likes(html: &ElementRef) -> Result<u32> {
    let like_selector = Selector::parse(r#"span[class="like_area _likeitArea"]"#)
        .expect("Failed to parse Like Selector");

    // Unsure what happens when a chapter has no likes
    let element = html
        .select(&like_selector)
        .next()
        .ok_or_else(|| anyhow!(format!("Failed to find likes element")))?;

    let chapter_number = element.text().collect::<Vec<_>>()[1].replace(',', "");

    let result = chapter_number
        .parse::<u32>()
        .with_context(|| format!("Failed to parse {chapter_number} to a u32"))?;

    Ok(result)
}

// TODO: Combine this with all other date selectors and just pass in the selector
fn date(html: &ElementRef) -> Result<String> {
    let date_selector = Selector::parse("span.date").expect("Failed to parse date Selector");

    let raw_date = html
        .select(&date_selector)
        .next()
        .ok_or_else(|| anyhow!("No date to parse"))?
        .text()
        .collect::<Vec<_>>()[0];

    let datetime = NaiveDate::parse_from_str(raw_date, "%b %e, %Y")
        .with_context(|| format!("Failed to parse {raw_date} to a date"))?;

    // %b %e, %Y -> Jun 3, 2022
    // %b %d, %Y -> Jun 03, 2022
    // %F -> 2022-06-03 (ISO 8601)
    let formatted = datetime.format("%F").to_string();

    Ok(formatted)
}

#[cfg(test)]
mod chapter_lists_parsing_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_chapter_number() {
        const NUMBER: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">

        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>


            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(NUMBER);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = 0;

        for chapter in html.select(&chapter_selector) {
            result = number(&chapter).unwrap();
        }

        assert_eq!(result, 24);
    }

    #[test]
    fn should_parse_chapter_likes() {
        const LIKES: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">

        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>


            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(LIKES);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = 0;

        for chapter in html.select(&chapter_selector) {
            result = likes(&chapter).unwrap();
        }

        assert_eq!(result, 7_779);
    }

    #[test]
    fn should_parse_chapter_date() {
        const DATE: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">

        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>


            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(DATE);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = String::new();

        for chapter in html.select(&chapter_selector) {
            result = date(&chapter).unwrap();
        }

        assert_eq!(result, "2022-11-20");
    }
}
