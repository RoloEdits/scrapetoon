use args::CliArgs;
use clap::*;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let path = args.output;
    let start = args.start;
    let end = args.end;
    let pages = args.pages;

    let parsed_chapters = tower_of_god::parse_chapters(start, end, pages).await;

    csv::write(&path, &parsed_chapters);
}
