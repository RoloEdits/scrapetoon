use core::time;
use std::{collections::LinkedList, thread};
use chrono::NaiveDate;
use cli_core::ProgressBarFactory;
use project_core::ResponseFactory;
use scraper::{Html, Selector, ElementRef};

use crate::ChapterInfo;

pub async fn parse_chapter_list_pages(end: u16, input_url: &str, chapter_info: &mut LinkedList<ChapterInfo>) {
    let bar = ProgressBarFactory::get_bar(end);

    for page in 1..=end {
        let url = format!("{}&page={}", input_url, page);

        let html_response = match ResponseFactory::get(&url).await {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("Error connecting to webpage, attempting to save progress and exit...");

                if chapter_info.is_empty() {
                    panic!("Nothing to save, exiting.");
                }

                break;
            }
        }
        .text()
        .await
        .unwrap();

        parse_each_chapters_chapter_info(&html_response, chapter_info);

        thread::sleep(time::Duration::from_secs(3));

        bar.inc(1);
    }
}

fn parse_each_chapters_chapter_info(html: &str, chapter_info: &mut LinkedList<ChapterInfo>) {
    let html = Html::parse_document(html);

    let chapter_selector = Selector::parse("ul#_listUl>li").unwrap();

    for chapter in html.select(&chapter_selector) {
        let chapter_number = parse_chapter_number(&chapter);
        let likes = parse_chapter_like_amount(&chapter);
        let date = parse_chapter_date(&chapter);
        chapter_info.push_back(ChapterInfo {
            chapter_number,
            likes,
            date,
        })
    }
}

fn parse_chapter_number(html: &ElementRef) -> u16 {
    let chapter_number_selector = Selector::parse("span.tx").unwrap();

    let mut result: u16 = 0;

    for element in html.select(&chapter_number_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[0];

        result = chapter_number.replace('#', "").parse::<u16>().unwrap();
    }

    result
}

fn parse_chapter_like_amount(html: &ElementRef) -> u32 {
    let like_selector = Selector::parse(r#"span[class="like_area _likeitArea"]"#).unwrap();

    let mut result: u32 = 0;

    for element in html.select(&like_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[1];

        result = chapter_number.replace(',', "").parse::<u32>().unwrap();
    }

    result
}

fn parse_chapter_date(html: &ElementRef) -> String {
    let date_selector = Selector::parse("span.date").unwrap();

    let mut holder: Vec<&str> = Vec::with_capacity(9);

    for element in html.select(&date_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[0];

        holder.push(chapter_number);
    }

    let mut result: String = String::new();

    for date in holder {
        let datetime = NaiveDate::parse_from_str(date, "%b %e, %Y").unwrap();

        // %b %e, %Y -> Jun 3, 2022
        // %b %d, %Y -> Jun 03, 2022
        // %F -> 2022-06-03 (ISO 8601)
        let formatted = datetime.format("%F").to_string();

        result = formatted;
    }

    result
}

#[cfg(test)]
mod chapter_lists_parsing_tests {
    use super::*;

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
            result = parse_chapter_number(&chapter);
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
            result = parse_chapter_like_amount(&chapter);
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
            result = parse_chapter_date(&chapter);
        }

        assert_eq!(result, "2022-11-20");
    }
}