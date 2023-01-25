use anyhow::{anyhow, Context, Result};
use project_core::ResponseFactory;
use scraper::{ElementRef, Html, Selector};
use std::collections::VecDeque;

use crate::DailyScheduleInfo;

const DAILY_SCHEDULE: &str = "https://www.webtoons.com/en/dailySchedule";

///# Panics
///
/// Will panic if there was a response but at the same time, the html text somehow didn't come with it unwrapping to a None.
#[tokio::main]
pub async fn parse() -> Result<VecDeque<DailyScheduleInfo>> {
    let mut series_list: VecDeque<DailyScheduleInfo> = VecDeque::new();

    let response = ResponseFactory::get(DAILY_SCHEDULE)
        .await
        .with_context(|| "Couldn't connect to Daily Schedule")?
        .text()
        .await
        .with_context(|| "Couldn't get text body from html response")?;

    let html = Html::parse_document(&response);

    parse_weekly_ongoing(&html, &mut series_list)?;
    parse_completed(&html, &mut series_list)?;

    Ok(series_list)
}

fn parse_weekly_ongoing(html: &Html, series_list: &mut VecDeque<DailyScheduleInfo>) -> Result<()> {
    let ongoing_selector = Selector::parse("div#dailyList>div.daily_section")
        .expect("Failed to parse Ongoing Selector");

    for week in html.select(&ongoing_selector) {
        parse_week(&week, series_list)?;
    }

    Ok(())
}

fn parse_completed(html: &Html, series_list: &mut VecDeque<DailyScheduleInfo>) -> Result<()> {
    let completed_selector =
        Selector::parse("div.comp>div.daily_section").expect("Failed to parse Completed Selector");

    for completed in html.select(&completed_selector) {
        parse_completed_cards(&completed, series_list)?;
    }

    Ok(())
}

fn parse_week(week: &ElementRef, series_list: &mut VecDeque<DailyScheduleInfo>) -> Result<()> {
    let day = parse_release_day(week)?;
    parse_weekly_cards(week, &day, series_list)?;
    Ok(())
}

fn parse_release_day(week: &ElementRef) -> Result<String> {
    let day_selector =
        Selector::parse("h2>a._weekdaySelect").expect("Failed to parse Day Selector");

    let day = week
        .select(&day_selector)
        .next()
        .ok_or_else(|| anyhow!("Failed to find release day"))?
        .value()
        .attr("data-weekday")
        .context("Failed to get value in 'data-weekday' field")?
        .to_string();

    Ok(day)
}

fn parse_weekly_cards(
    week: &ElementRef,
    day: &str,
    series_list: &mut VecDeque<DailyScheduleInfo>,
) -> Result<()> {
    let card_list_selector = Selector::parse("ul.daily_card>li").unwrap();

    for card in week.select(&card_list_selector) {
        let title = parse_daily_schedule_title(&card);
        let author = parse_daily_schedule_author(&card);
        let genre = parse_daily_schedule_genre(&card);
        let total_likes = parse_daily_schedule_total_likes(&card)?;
        let status = parse_daily_schedule_is_completed(&card);

        let day = match day {
            "SUNDAY" => "Sunday",
            "MONDAY" => "Monday",
            "TUESDAY" => "Tuesday",
            "WEDNESDAY" => "Wednesday",
            "THURSDAY" => "Thursday",
            "FRIDAY" => "Friday",
            "SATURDAY" => "Saturday",
            _ => "",
        }
        .to_string();

        series_list.push_back(DailyScheduleInfo {
            title,
            author,
            genre,
            total_likes,
            status,
            day,
        });
    }

    Ok(())
}

fn parse_completed_cards(
    completed: &ElementRef,
    series_list: &mut VecDeque<DailyScheduleInfo>,
) -> Result<()> {
    let card_list_selector = Selector::parse("ul.daily_card>li").unwrap();

    for card in completed.select(&card_list_selector) {
        let title = parse_daily_schedule_title(&card);
        let author = parse_daily_schedule_author(&card);
        let genre = parse_daily_schedule_genre(&card);
        let total_likes = parse_daily_schedule_total_likes(&card)?;
        let status = parse_daily_schedule_is_completed(&card);

        series_list.push_back(DailyScheduleInfo {
            title,
            author,
            genre,
            total_likes,
            status,
            day: "Completed".to_string(),
        });
    }

    Ok(())
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

fn parse_daily_schedule_total_likes(card: &ElementRef) -> Result<u32> {
    let likes_selector = Selector::parse("em.grade_num").expect("Failed to parse likes selector");

    let mut to_check = String::new();

    for likes in card.select(&likes_selector) {
        to_check = likes.text().collect::<Vec<_>>()[0].to_string();
    }

    let result = match to_check {
        m if m.ends_with('M') => {
            let cleaned_m = m.replace('M', "");

            let millions = cleaned_m
                .parse::<f32>()
                .with_context(|| format!("Failed to parse {cleaned_m} to a f32"))?
                * 1_000_000.0;
            millions as u32
        }
        k => {
            let cleaned_k = k.replace(',', "");

            cleaned_k
                .parse::<u32>()
                .with_context(|| format!("Failed to parse {cleaned_k} to a u32"))?
        }
    };

    Ok(result)
}

fn parse_daily_schedule_genre(card: &ElementRef) -> String {
    let genre_selector = Selector::parse("p.genre").expect("Failed to parse genre selector");

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
    let title_selector = Selector::parse("p.subj").expect("Failed to parse Title Selector");

    let mut result = String::new();

    for title in card.select(&title_selector) {
        result = title.text().collect::<Vec<_>>()[0].to_string();
    }

    result
}

#[cfg(test)]
mod daily_schedule_parsing_tests {
    //    use super::*;
    // use pretty_assertions::assert_eq;
}
