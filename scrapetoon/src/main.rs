use args::{LineArgs, SourceData};
use clap::*;
use project_core::create_date_folder;
use std::path::Path;

mod args;
mod csv;

fn main() {
    let cli = LineArgs::parse();

    match cli.source {
        SourceData::Daily { output } => {
            let date_path = create_date_folder(&output);

            if !Path::new(&date_path).exists() {
                eprintln!("Error! Invalid output path!");
                return;
            }

            println!("Connecting to Daily Schedule...");
            let daily = line_core::parse::parse_daily_schedule();
            csv::write_daily_schedule(&date_path, &daily);
            println!("Finshed scraping Daily Schedule!")
        }
        SourceData::Story { url, output, end } => {
            let date_path = create_date_folder(&output);

            if !Path::new(&date_path).exists() {
                eprintln!("Error! Invalid output path!");
                return;
            }

            println!("Connecting to Story Page...");
            let info = line_core::parse::series_info(end, &url);
            csv::write_series_info(&date_path, &info);
            println!("Finshed scraping {}!", info.title)
        }
    }
}
