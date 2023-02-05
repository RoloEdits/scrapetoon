mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::{CsvWrite, IntoStoryRecord, Webtoons};

pub const TO_SKIP: fn(u16) -> bool = |chapter: u16| -> bool {
    // The URl no=221 for chapter 221 is a 404. No=222 is where #221 is.
    matches!(chapter, 221)
};

const PAGE_URL: &str = "https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95";

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
        Some(4),
    )?;

    story.into_record().write(&args.output, &kebab_title)?;

    Ok(())
}
