mod args;
mod parsing;

use anyhow::{bail, Result};
use args::{Scrapetoon, Source};
use clap::Parser;
use std::path::Path;
use tracing::info;
use webtoons::story::csv::models::IntoStoryRecord;
use webtoons::utils::{create_date_folder, CsvWrite};

const TO_SKIP: fn(u16) -> bool = |_chapter: u16| -> bool { false };

fn main() -> Result<()> {
    let cli = Scrapetoon::parse();
    tracing_subscriber::fmt::init();

    match cli.source {
        Source::Daily { path } => {
            let date_path = create_date_folder(&path)?;

            if !Path::new(&date_path).exists() {
                bail!("Invalid output path")
            }

            info!("Connecting to Daily Schedule...");
            let daily = webtoons::parse_daily_schedule()?;
            daily.write(&date_path, "daily-schedule.csv")?;
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
        } => {
            let date_path = create_date_folder(&path)?;

            info!("Connecting to Story Page...");
            let (story, kebab_title) = webtoons::parse_series(
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
                threads,
            )?;

            story.into_record().write(&date_path, &kebab_title)?;
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
            webtoons::download_panels(&url, &path, start, end, threads)?;
            info!("Finished Downloading Panels!");
        }
    }

    Ok(())
}
