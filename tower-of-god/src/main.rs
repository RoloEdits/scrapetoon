use clap::Parser;
use cli_core::StoryCliArgs;
use project_core::enforce_path_exists;
use tower_of_god::config;

mod csv;

#[tokio::main]
async fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = tower_of_god::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(
        &enforce_path_exists(&args.output),
        &parsed_chapters,
        &series_info,
        config::CONFIG.filename,
    );
}
