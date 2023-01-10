use chrono::NaiveDate;
use crossbeam::queue::SegQueue;
use indicatif::ParallelProgressIterator;
use project_core::ResponseFactory;
use rayon::prelude::*;
use scraper::{ElementRef, Html, Selector};

use std::collections::LinkedList;

use crate::ChapterListInfo;

///# Panics
///
/// Will panic if there was a response but at the same time, the html text somehow didn't come with it unwrapping to a None.
#[must_use]
pub fn parse(end: u16, input_url: &str) -> LinkedList<ChapterListInfo> {
    // 8 Threads is around the line at which problems start to occur when pinging out too many times at once as all getting blocked
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();

    let range: Vec<_> = (1..=end).collect();
    let total = range.len() as u64;

    let chapter_info: SegQueue<ChapterListInfo> = SegQueue::new();

    range
        .into_par_iter()
        .progress_count(total)
        .for_each(|page| {
            let url = format!("{input_url}&page={page}");
            work(&url, &chapter_info);
        });

    let mut result: LinkedList<ChapterListInfo> = LinkedList::new();

    for info in chapter_info {
        result.push_back(info);
    }

    result
}

#[tokio::main]
async fn work(url: &str, chapter_info: &SegQueue<ChapterListInfo>) {
    if let Ok(response) = ResponseFactory::get(url).await {
        let html = response.text().await.unwrap();

        parse_each_chapters_chapter_info(&html, chapter_info);
    };
}

fn parse_each_chapters_chapter_info(html: &str, chapter_info: &SegQueue<ChapterListInfo>) {
    let html = Html::parse_document(html);

    let chapter_selector = Selector::parse("ul#_listUl>li").unwrap();

    for chapter in html.select(&chapter_selector) {
        let chapter_number = parse_chapter_number(&chapter);
        let likes = parse_chapter_like_amount(&chapter);
        let date = parse_chapter_date(&chapter);
        chapter_info.push(ChapterListInfo {
            chapter_number,
            likes,
            date,
        });
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
