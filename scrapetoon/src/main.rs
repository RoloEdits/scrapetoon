use anyhow::{bail, Result};
use args::{Scrapetoon, Source};
use clap::Parser;
use project_core::create_date_folder;
use std::path::Path;

mod args;
mod csv;

fn main() -> Result<()> {
    let cli = Scrapetoon::parse();

    match cli.source {
        Source::Daily { path: output } => {
            let date_path = create_date_folder(&output)?;

            if !Path::new(&date_path).exists() {
                bail!("Invalid output path")
            }

            println!("Connecting to Daily Schedule...");
            let daily = line_core::daily_schedule::parse()?;
            csv::write_daily_schedule(&date_path, &daily);
            println!("Finished scraping Daily Schedule!");
        }
        Source::Page {
            path: output,
            url,
            end,
        } => {
            let date_path = create_date_folder(&output)?;

            println!("Connecting to Story Page...");
            let info = line_core::series_info::parse(end, &url)?;
            csv::write_series_info(&date_path, &info);
            println!("Finished scraping {}!", info.title);
        }
        Source::Panels {
            path,
            url,
            start,
            end,
        } => {
            println!("Connecting...");
            line_core::panels::get(&url, &path, start, end)?;
            println!("Finished Downloading Panels!");
        }
    }

    Ok(())
}
