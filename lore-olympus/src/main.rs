use clap::Parser;
use cli_core::StoryCliArgs;
use lore_olympus::config;
use project_core::path_enforcer;

mod csv;

#[tokio::main]
async fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = lore_olympus::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(
        &path_enforcer(&args.output),
        &parsed_chapters,
        &series_info,
        config::CONFIG.filename,
    );
}
