use hashlink::LinkedHashSet;
use project_core::ResponseFactory;
use scraper::{ElementRef, Html, Selector};

use crate::DailyScheduleInfo;

///# Panics
///
/// Will panic if there was a response but at the same time, the html text somehow didn't come with it unwrapping to a None.
pub async fn parse() -> LinkedHashSet<DailyScheduleInfo> {
    const DAILY_SCHEDULE: &str = "https://www.webtoons.com/en/dailySchedule";

    let mut series: LinkedHashSet<DailyScheduleInfo> = LinkedHashSet::new();

    let html = ResponseFactory::get(DAILY_SCHEDULE)
        .await
        .map_or_else(
            |_| panic!("Error connecting to URL webpage: {DAILY_SCHEDULE}"),
            |html_response| html_response,
        )
        .text()
        .await
        .expect("Error getting HTML from response");

    let html = Html::parse_document(&html);
    let daily_card = Selector::parse("ul.daily_card>li").unwrap();

    for card in html.select(&daily_card) {
        let title = parse_daily_schedule_title(&card);
        let author = parse_daily_schedule_author(&card);
        let genre = parse_daily_schedule_genre(&card);
        let total_likes = parse_daily_schedule_total_likes(&card);
        let status = parse_daily_schedule_is_completed(&card);

        series.insert(DailyScheduleInfo {
            title,
            author,
            genre,
            total_likes,
            status,
        });
    }
    series
}

fn parse_daily_schedule_is_completed(card: &ElementRef) -> String {
    let completed_selector = Selector::parse("p.icon_area").unwrap();

    let mut result = String::new();

    for status_check in card.select(&completed_selector) {
        let holder = status_check.text().collect::<Vec<_>>();

        if holder.is_empty() {
            return "ongoing".to_string();
        }

        result = holder[0].to_string();
    }

    match result {
        hiatus if hiatus == "HIATUS" => "hiatus".to_string(),
        completed if completed == "COMPLETED" => "completed".to_string(),
        _ => "ongoing".to_string(),
    }
}

fn parse_daily_schedule_total_likes(card: &ElementRef) -> u32 {
    let likes_selector = Selector::parse("em.grade_num").unwrap();

    let mut result = String::new();

    for likes in card.select(&likes_selector) {
        result = likes.text().collect::<Vec<_>>()[0].to_string();
    }

    match result {
        sub_text if sub_text.ends_with('M') => {
            let millions = sub_text
                .replace('M', "")
                .parse::<f32>()
                .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={sub_text}"))
                * 1_000_000.0;
            millions as u32
        }
        sub_text => sub_text
            .replace(',', "")
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={sub_text}")),
    }
}

fn parse_daily_schedule_genre(card: &ElementRef) -> String {
    let genre_selector = Selector::parse("p.genre").unwrap();

    let mut result = String::new();

    for genre in card.select(&genre_selector) {
        result = genre.text().collect::<Vec<_>>()[0].to_string();
    }

    result
}

fn parse_daily_schedule_author(card: &ElementRef) -> String {
    let author_selector = Selector::parse("p.author").unwrap();

    let mut result = String::new();

    for author in card.select(&author_selector) {
        result = author.text().collect::<Vec<_>>()[0].to_string();
    }

    result
}

fn parse_daily_schedule_title(card: &ElementRef) -> String {
    let title_selector = Selector::parse("p.subj").unwrap();

    let mut result = String::new();

    for title in card.select(&title_selector) {
        result = title.text().collect::<Vec<_>>()[0].to_string();
    }

    result
}

#[cfg(test)]
mod daily_schedule_parsing_tests {
    //    use super::*;
}
