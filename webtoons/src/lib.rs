pub mod daily;
mod factories;
pub mod story;
pub mod utils;

use anyhow::Result;
use daily::models::Daily;
use scraper::Html;
use story::models::Story;
use tracing::info;

type Season = fn(&Html, u16) -> Option<u8>;
type SeasonChapter = fn(&Html, u16) -> Option<u16>;
type Arc = fn(&Html, u16) -> Option<String>;
pub type Skip = fn(u16) -> bool;

/// # Errors
pub fn parse_daily_schedule() -> Result<Vec<Daily>> {
    info!("Starting Daily Schedule Parsing");
    let daily_schedule = daily::parse()?;
    info!("Finished Daily Schedule Parsing");
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
    info!("Starting Story Parsing");
    let (series_info, kebab_title) =
        story::parse(start, end, url, season, season_chapter, arc, skip)?;
    info!("Finished Story Parsing");
    Ok((series_info, kebab_title))
}

/// # Errors
pub fn download_panels(url: &str, path: &str, start: u16, end: u16) -> Result<()> {
    info!("Starting Panel Download");
    story::chapter::panels::get(url, path, start, end)?;
    info!("Finished Panel Download");
    Ok(())
}
