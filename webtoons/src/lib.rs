pub mod daily;
mod factories;
pub mod story;
pub mod utils;

use anyhow::Result;
use daily::models::Daily;
use scraper::Html;
use std::collections::VecDeque;
use story::models::Story;

type Season = fn(&Html, u16) -> Option<u8>;
type SeasonChapter = fn(&Html, u16) -> Option<u16>;
type Arc = fn(&Html, u16) -> Option<String>;
pub type Skip = fn(u16) -> bool;

/// # Errors
pub fn parse_daily_schedule() -> Result<VecDeque<Daily>> {
    let daily_schedule = daily::parse()?;
    Ok(daily_schedule)
}

/// # Errors
pub fn parse_series(
    start: u16,
    end: u16,
    url: &str,
    season: Season,
    season_chapter: SeasonChapter,
    arc: Arc,
    skip: Skip,
) -> Result<(Story, String)> {
    let (series_info, kebab_title) =
        story::parse(start, end, url, season, season_chapter, arc, skip)?;
    Ok((series_info, kebab_title))
}

/// # Errors
pub fn download_panels(url: &str, path: &str, start: u16, end: u16) -> Result<()> {
    story::chapter::panels::get(url, path, start, end)?;
    Ok(())
}
