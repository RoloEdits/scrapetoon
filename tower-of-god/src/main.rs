use args::CliArgs;
use clap::*;
use tower_of_god::config;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let (series_info, parsed_chapters) = tower_of_god::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(&args.output, &parsed_chapters, &series_info, config::CONFIG.filename);
}
