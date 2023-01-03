use clap::Parser;
use cli_core::StoryCliArgs;
use true_beauty::config;
use project_core::validate_output_path_ends_correctly;

mod csv;

#[tokio::main]
async fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = true_beauty::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(
        &validate_output_path_ends_correctly(&args.output),
        &parsed_chapters,
        &series_info,
        config::CONFIG.filename,
    );
}
