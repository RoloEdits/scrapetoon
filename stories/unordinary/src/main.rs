mod parsing;

use anyhow::Result;
use clap::Parser;
use cli::StoryCliArgs;
use webtoons::story::csv::models::IntoStoryRecord;
use webtoons::utils;
use webtoons::utils::CsvWrite;

const TO_SKIP: fn(u16) -> bool = |chapter: u16| -> bool {
    matches!(
        chapter,
        1 | 37 | 48 | 80 | 81 | 82 | 162 | 163 | 164 | 165 | 166 | 246 | 247
    )
};

const PAGE_URL: &str = "https://www.webtoons.com/en/super-hero/unordinary/list?title_no=679";

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
    )?;

    let path = utils::path_enforcer(&args.output)?;

    story.into_record().write(path, &kebab_title)?;

    Ok(())
}
