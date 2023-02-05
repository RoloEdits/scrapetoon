mod args;
mod parsing;

use anyhow::Result;
use args::{Scrapetoon, Source};
use clap::Parser;
use tracing::info;
use webtoons::{CsvWrite, IntoStoryRecord, Webtoons};

const TO_SKIP: fn(u16) -> bool = |_chapter: u16| -> bool { false };

fn main() -> Result<()> {
    let cli = Scrapetoon::parse();
    tracing_subscriber::fmt::init();

    match cli.source {
        Source::Daily { path } => {
            info!("Connecting to Daily Schedule...");
            let daily = Webtoons::parse_daily_schedule()?;
            daily.write(&path, "daily-schedule.csv")?;
            info!("Finished scraping Daily Schedule!");
        }
        Source::Story {
            path,
            url,
            start,
            end,
            pages,
            completed,
            threads,
            top_comments,
            all_comments,
        } => {
            info!("Connecting to Story Page...");
            let (story, kebab_title) = Webtoons::parse_series(
                start,
                end,
                pages,
                &url,
                parsing::season,
                parsing::season_chapter,
                parsing::arc,
                parsing::custom,
                TO_SKIP,
                completed,
                top_comments,
                all_comments,
                threads,
            )?;

            story.into_record().write(&path, &kebab_title)?;

            info!("Finished scraping {kebab_title}!");
        }
        Source::Panels {
            path,
            url,
            start,
            end,
            threads,
        } => {
            info!("Connecting...");
            Webtoons::download_panels(&url, &path, start, end, threads)?;
            info!("Finished Downloading Panels!");
        }
    }

    Ok(())
}
