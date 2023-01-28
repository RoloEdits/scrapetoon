#![allow(unused)]

use scraper::{Html, Selector};
use webtoons::regex;

pub fn season(html: Option<&Html>, chapter: u16) -> Option<u8> {
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").expect("Invalid title selector");

        let regex = regex![r"\(S(\d)\)"];

        let title = html
            .select(&title_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()[0];

        let season = match regex.captures(title) {
            Some(cap) => cap,
            None => return Some(1),
        }
        .get(1)?
        .as_str()
        .parse::<u8>()
        .expect("Failed to parse season number from title");

        return Some(season);
    }

    None
}

pub const fn season_chapter(html: Option<&Html>, chapter: u16) -> Option<u16> {
    if let Some(html) = html {}

    None
}

pub const fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    None
}

pub fn custom(html: Option<&Html>, chapter: u16) -> Option<u16> {
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();

        let regex = regex![r"Episode\s(\d+)"];

        let title = html
            .select(&title_selector)
            .into_iter()
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0];

        let meaningful_chapter_number = regex
            .captures(title)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();

        return Some(meaningful_chapter_number);
    }

    None
}

#[cfg(test)]
mod lore_olympus_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_produce_season_number() {
        const SEASON_NUMBER: &str = r#"<div class="subj_info">
						<a href="https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320" class="subj NPI=a:end,g:en_en" title="Lore Olympus">Lore Olympus</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="(S3) Episode 225">(S3) Episode 225</h1>
					</div>"#;

        const SEASON_NUMBER2: &str = r#"<div class="subj_info">
						<a href="https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320" class="subj NPI=a:end,g:en_en" title="Lore Olympus">Lore Olympus</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Episode 1">Episode 1</h1>
					</div>"#;

        let html = Html::parse_document(SEASON_NUMBER);
        let html2 = Html::parse_document(SEASON_NUMBER2);

        let season_number = season(Some(&html), 0).unwrap();
        let season_number2 = season(Some(&html2), 0).unwrap();

        assert_eq!(season_number, 3);
        assert_eq!(season_number2, 1);
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

    #[test]
    fn should_produce_custom() {
        const CHAPTER_NUMBER1: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320" class="subj NPI=a:end,g:en_en" title="Lore Olympus">Lore Olympus</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Episode 1">Episode 1</h1>
					</div>"##;

        const CHAPTER_NUMBER2: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/romance/lore-olympus/list?title_no=1320" class="subj NPI=a:end,g:en_en" title="Lore Olympus">Lore Olympus</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="(S2) Episode 116 (Season 2 Premiere)">(S2) Episode 116 (Season 2 Premiere)</h1>
					</div>"##;

        let html1 = Html::parse_document(CHAPTER_NUMBER1);
        let html2 = Html::parse_document(CHAPTER_NUMBER2);

        let result1 = custom(Some(&html1), 1).unwrap();
        let result2 = custom(Some(&html2), 116).unwrap();

        assert_eq!(result1, 1);
        assert_eq!(result2, 116);
    }
}
