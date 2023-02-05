mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::{CsvWrite, IntoStoryRecord, Webtoons};

const PAGE_URL: &str = "https://www.webtoons.com/en/romance/truebeauty/list?title_no=1436";

const TO_SKIP: fn(u16) -> bool = |_chapter: u16| -> bool { false };

fn main() -> Result<()> {
    let args = StoryCliArgs::parse();
    tracing_subscriber::fmt::init();

    let (story, kebab_title) = Webtoons::parse_series(
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
        Some(12),
    )?;

    story.into_record().write(&args.output, &kebab_title)?;

    Ok(())
}
