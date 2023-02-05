mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::{CsvWrite, IntoStoryRecord, Webtoons};

// #13: Message and Concept Art
// #18: Q&A
// #31: Hiatus special short
// #47: QnA
// #120: Season 1 Recap
pub const TO_SKIP: fn(u16) -> bool =
    |chapter: u16| -> bool { matches!(chapter, 13 | 18 | 31 | 47 | 120) };

const PAGE_URL: &str = "https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320";

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
