#![allow(unused)]

use scraper::{Html, Selector};
use webtoons::parsing;

pub const fn season(html: Option<&Html>, chapter: u16) -> Option<u8> {
    if let Some(html) = html {}
    None
}

pub const fn season_chapter(html: Option<&Html>, chapter: u16) -> Option<u16> {
    if let Some(html) = html {}
    None
}

pub const fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    if let Some(html) = html {}
    None
}

pub fn custom(html: Option<&Html>, chapter: u16) -> Option<u16> {
    if let Some(html) = html {
        let chapter_number = parsing::parse_title(html, r"Episode\s(\d+)")?
            .parse::<u16>()
            .unwrap();

        return Some(chapter_number);
    }

    None
}

#[cfg(test)]
mod unordinary_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_produce_custom() {
        const CHAPTER_NUMBER1: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/super-hero/unordinary/list?title_no=679" class="subj NPI=a:end,g:en_en" title="unOrdinary">unOrdinary</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Episode 78">Episode 78</h1>
					</div>"##;

        let html1 = Html::parse_document(CHAPTER_NUMBER1);

        let result1 = custom(Some(&html1), 78).unwrap();

        assert_eq!(result1, 78);
    }

    #[test]
    #[ignore]
    fn should_produce_season_number() {
        todo!()
    }
    #[test]
    #[ignore]
    fn should_produce_season_chapter_number() {
        todo!()
    }
    #[test]
    #[ignore]
    fn should_produce_arc() {
        todo!()
    }
}
