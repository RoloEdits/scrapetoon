use args::CliArgs;
use clap::*;
use the_god_of_high_school::config;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let (series_info, parsed_chapters) = the_god_of_high_school::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(&args.output, &parsed_chapters, &series_info, config::CONFIG.filename);
}
