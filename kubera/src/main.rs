use clap::Parser;
use cli_core::StoryCliArgs;
use kubera::config;
use project_core::path_enforcer;

mod csv;

fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = kubera::parse_chapters(
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
