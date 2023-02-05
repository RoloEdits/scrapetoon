mod daily;
mod factories;
mod story;
mod utils;

use anyhow::{Context, Result};
use daily::models::Daily;
use scraper::Html;
use story::models::Story;
use tracing::info;

// Exporting
pub use story::csv::models::IntoStoryRecord;
pub use utils::validate_path;
pub use utils::CsvWrite;

type Season = fn(Option<&Html>, u16) -> Option<u8>;
type SeasonChapter = fn(Option<&Html>, u16) -> Option<u16>;
type Arc = fn(Option<&Html>, u16) -> Option<String>;
type Custom<T> = fn(Option<&Html>, u16) -> Option<T>;
type SkipChapter = fn(u16) -> bool;

pub struct Webtoons;

impl Webtoons {
    /// # Errors
    pub fn parse_daily_schedule() -> Result<Vec<Daily>> {
        info!("Starting Daily Schedule Parsing");
        let daily_schedule = daily::parse()?;
        info!("Finished Daily Schedule Parsing");
        Ok(daily_schedule)
    }

    // Just one over the limit and, for now, it is easier to follow by having explicit types and names in the argument list
    #[allow(clippy::too_many_arguments)]
    /// # Errors
    pub fn parse_series<T: Clone + Send>(
        start: u16,
        end: u16,
        pages: u16,
        url: &str,
        season: Season,
        season_chapter: SeasonChapter,
        arc: Arc,
        custom: Custom<T>,
        skip: SkipChapter,
        is_completed: bool,
        top_comments: bool,
        all_comments: bool,
        threads: Option<usize>,
    ) -> Result<(Story<T>, String)> {
        info!("Starting Story Parsing");
        // 6 Threads is around the line at which problems start to occur when pinging out too many times at once as all getting blocked
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.unwrap_or(4))
            .build_global()
            .context("Couldn't create thread pool")?;

        if is_completed {
            let (series_info, kebab_title) = story::parse(
                start,
                end,
                url,
                season,
                season_chapter,
                arc,
                custom,
                skip,
                is_completed,
                top_comments,
                all_comments,
                None,
            )?;
            info!("Finished Story Parsing");
            return Ok((series_info, kebab_title));
        }

        let publish_map = story::chapter_list::parse(pages, url)?;

        let (series_info, kebab_title) = story::parse(
            start,
            end,
            url,
            season,
            season_chapter,
            arc,
            custom,
            skip,
            is_completed,
            top_comments,
            all_comments,
            Some(&publish_map),
        )?;
        info!("Finished Story Parsing");
        Ok((series_info, kebab_title))
    }

    /// # Errors
    pub fn download_panels(
        url: &str,
        path: &str,
        start: u16,
        end: u16,
        threads: Option<usize>,
    ) -> Result<()> {
        // 6 Threads is around the line at which problems start to occur when pinging out too many times at once as all getting blocked
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.unwrap_or(6))
            .build_global()
            .context("Failed to build thread pool")?;

        info!("Starting Panel Download");
        story::chapter::panels::get(url, path, start, end)?;
        info!("Finished Panel Download");
        Ok(())
    }
}

pub mod parsing {
    use crate::regex;
    use scraper::{Html, Selector};

    /// # Panics
    pub fn parse_title(html: &Html, pattern: &str) -> Option<String> {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();

        let regex = regex![pattern];

        let title = html
            .select(&title_selector)
            .into_iter()
            .next()?
            .text()
            .collect::<Vec<_>>()[0];

        let parsed = regex.captures(title).unwrap().get(1)?.as_str().to_owned();

        Some(parsed)
    }
}
