pub mod comments;
mod length;
mod likes;
pub mod models;
pub mod panels;

use crate::factories::BlockingReferClient;
use crate::story::chapter::models::Chapter;
use crate::{utils, Arc, Season, SeasonChapter, SkipChapter};
use anyhow::{bail, Context, Result};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tracing::error;

#[allow(clippy::too_many_arguments)]
/// # Errors
pub fn parse(
    start: u16,
    end: u16,
    id: u32,
    season: Season,
    season_chapter: SeasonChapter,
    arc: Arc,
    skip_chapter: SkipChapter,
    is_completed: bool,
    chapter_published: Option<&HashMap<u16, String>>,
) -> Result<Vec<Chapter>> {
    let chapters: Vec<_> = (start..=end).collect();
    let total = chapters.len() as u64;

    // TODO: Figure out proper error propagation
    let vec = chapters
        .into_par_iter()
        .progress_count(total)
        .filter_map(|chap| {
            match chapter(
                id,
                chap,
                season,
                season_chapter,
                arc,
                skip_chapter,
                is_completed,
                chapter_published,
            ) {
                Ok(ok) => ok,
                Err(err) => {
                    error!("Failed to parse chapter {chap} due to {err}");
                    None
                }
            }
        })
        .collect();

    Ok(vec)
}

// Just one over the limit and, for now, it is easier to follow by having explicit types and names in the argument list
#[allow(clippy::too_many_arguments)]
fn chapter(
    id: u32,
    chapter: u16,
    season_fn: Season,
    season_chapter_fn: SeasonChapter,
    arc_fn: Arc,
    skip_chapter: SkipChapter,
    is_completed: bool,
    chapter_published: Option<&HashMap<u16, String>>,
) -> Result<Option<Chapter>> {
    if skip_chapter(chapter) {
        return Ok(None);
    }

    let mut number: Option<u16> = None;
    let mut length: Option<u32> = None;
    let mut published: Option<String> = None;
    let mut html: Option<Html> = None;

    if !is_completed {
        let url = chapter_url(id, chapter);

        let response = BlockingReferClient::get(&url).unwrap();

        if response.status() != StatusCode::OK {
            return Ok(None);
        }

        let text = response.text()?;

        let temp_html = Html::parse_document(&text);

        let temp_number = chapter_number(&temp_html)?;

        let get_date = chapter_published
            .expect("No HashMap for date given")
            .get(&temp_number)
            .unwrap()
            .clone();

        number = Some(temp_number);
        length = Some(length::parse(&temp_html)?);
        published = Some(get_date);
        html = Some(temp_html);
    }

    let season = season_fn(html.as_ref(), number.unwrap_or(chapter));
    let season_chapter = season_chapter_fn(html.as_ref(), number.unwrap_or(chapter));
    let arc = arc_fn(html.as_ref(), number.unwrap_or(chapter));

    let likes = likes::parse(id, chapter)?;

    // To handle chapter misalignment, comment::parse() needs to use the passed in `chapter`, otherwise this will grab data from the bad chapter
    // as the chapter number parsed from whats displayed is contiguous, causing the shift to happen
    let (comments, replies, user_comments) = comments::parse(id, chapter)?;

    let utc = utils::get_current_utc_date_verbose();

    let result = Chapter {
        number: number.unwrap_or(chapter),
        likes,
        length,
        comments,
        replies,
        season,
        season_chapter,
        arc,
        user_comments,
        published,
        scraped: utc,
    };

    Ok(Some(result))
}

fn chapter_number(html: &Html) -> Result<u16> {
    if let Ok(chapter_number_selector) = Selector::parse("span.tx") {
        if let Some(chapter_number) = html.select(&chapter_number_selector).next() {
            // .ok_or_else(|| anyhow!("Should find a chapter number"))?
            let vec = chapter_number.text().collect::<Vec<_>>();

            let text = vec
                .first()
                .expect("Failed to select first occurrence of chapter number");

            let cleaned = text.replace('#', "");

            let result = cleaned
                .parse::<u16>()
                .with_context(|| format!("Failed to parse {cleaned} to a u16"))?;

            return Ok(result);
        }
    }

    bail!("Failed to create chapter number selector")
}

fn chapter_url(id: u32, chapter: u16) -> String {
    format!("https://www.webtoons.com/en/*/*/*/viewer?title_no={id}&episode_no={chapter}")
}

#[cfg(test)]
mod parse_comments_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_chapter_number() {
        const CHAPTER_NUMBER: &str =
            r##"<span class="tx _btnOpenEpisodeList NPI=a:current,g:en_en">#550</span>"##;

        let html = Html::parse_document(CHAPTER_NUMBER);

        let result = chapter_number(&html).unwrap();

        assert_eq!(result, 550);
    }

    #[test]
    fn should_form_valid_url() {
        const CHAPTER_NUMBER: &str =
            "https://www.webtoons.com/en/*/*/*/viewer?title_no=95&episode_no=2";

        let result = chapter_url(95, 2);

        assert_eq!(CHAPTER_NUMBER, result);
    }
}
