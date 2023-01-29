mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::story::csv::models::IntoStoryRecord;
use webtoons::utils;
use webtoons::utils::CsvWrite;

pub const TO_SKIP: fn(u16) -> bool =
    |chapter: u16| -> bool { matches!(chapter, |102| 267 | 284 | 285) };

const PAGE_URL: &str = "https://www.webtoons.com/en/fantasy/kubera/list?title_no=83";

fn main() -> Result<()> {
    let args = StoryCliArgs::parse();
    tracing_subscriber::fmt::init();

    let (story, kebab_title) = webtoons::parse_series(
        args.start,
        args.end,
        args.pages,
        PAGE_URL,
        parsing::season,
        parsing::season_chapter,
        parsing::arc,
        parsing::custom,
        TO_SKIP,
        false,
        Some(4), // Any more than 4 threads is too fast and will more probably than not face timout issues
    )?;

    let path = utils::path_enforcer(&args.output)?;

    story.into_record().write(path, &kebab_title)?;

    Ok(())
}
