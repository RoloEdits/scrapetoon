mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::story::csv::models::IntoStoryRecord;
use webtoons::utils;
use webtoons::utils::CsvWrite;

const TO_SKIP: fn(u16) -> bool = |chapter: u16| -> bool { matches!(chapter, 249 | 204 | 309) };

const PAGE_URL: &str = "https://www.webtoons.com/en/action/the-god-of-high-school/list?title_no=66";

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
        args.top_comments,
        args.all_comments,
        Some(0),
    )?;

    let path = utils::path_enforcer(&args.output)?;

    story.into_record().write(path, &kebab_title)?;

    Ok(())
}
