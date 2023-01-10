use clap::Parser;
use cli_core::StoryCliArgs;
use project_core::path_enforcer;
use true_beauty::config;

mod csv;

fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = true_beauty::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    );

    csv::write(
        path_enforcer(&args.output),
        &parsed_chapters,
        &series_info,
        config::CONFIG.filename,
    );
}
