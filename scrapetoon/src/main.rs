mod parsing;

use ::csv::CsvWrite;
use anyhow::{bail, Result};
use args::{Scrapetoon, Source};
use clap::Parser;
use csv::story::IntoStoryRecord;
use std::path::Path;
use tracing::info;
use webtoons::utils::create_date_folder;

mod args;

const TO_SKIP: fn(u16) -> bool = |_chapter: u16| -> bool { false };

fn main() -> Result<()> {
    let cli = Scrapetoon::parse();

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
        } => {
            let date_path = create_date_folder(&path)?;

            info!("Connecting to Story Page...");
            let (story, kebab_title) = webtoons::parse_series(
                start,
                end,
                &url,
                parsing::season,
                parsing::season_chapter,
                parsing::arc,
                TO_SKIP,
            )?;
            story.into_record().write(&date_path, &kebab_title)?;
            info!("Finished scraping {kebab_title}!");
        }
        Source::Panels {
            path,
            url,
            start,
            end,
        } => {
            info!("Connecting...");
            webtoons::story::chapter::panels::get(&url, &path, start, end)?;
            info!("Finished Downloading Panels!");
        }
    }

    Ok(())
}
