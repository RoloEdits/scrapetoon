use args::{Scrapetoon, SourceData};
use clap::Parser;
use project_core::create_date_folder;
use std::path::Path;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let cli = Scrapetoon::parse();

    match cli.source {
        SourceData::Daily { output } => {
            let date_path = create_date_folder(&output);

            if !Path::new(&date_path).exists() {
                eprintln!("Error! Invalid output path!");
                return;
            }

            println!("Connecting to Daily Schedule...");
            let daily = line_core::daily_schedule::parse();
            csv::write_daily_schedule(&date_path, &daily.await);
            println!("Finished scraping Daily Schedule!");
        }
        SourceData::Story { url, output, end } => {
            let date_path = create_date_folder(&output);

            if !Path::new(&date_path).exists() {
                eprintln!("Error! Invalid output path!");
                return;
            }

            println!("Connecting to Story Page...");
            let info = line_core::series_info::parse(end, &url).await;
            csv::write_series_info(&date_path, &info);
            println!("Finished scraping {}!", info.title);
        }
    }
}
