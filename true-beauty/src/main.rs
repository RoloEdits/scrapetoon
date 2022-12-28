use args::CliArgs;
use clap::*;
use true_beauty::config;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let parsed_chapters = true_beauty::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(&args.output, &parsed_chapters, config::CONFIG.filename);
}
