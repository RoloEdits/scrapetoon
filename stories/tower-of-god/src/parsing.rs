#![allow(unused)]

use scraper::{Html, Selector};
use webtoons::regex;

pub fn season(html: Option<&Html>, chapter: u16) -> Option<u8> {
    // input eg. '[Season 3] Ep. 133'
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();
        // Season (3) where 3 is captured as 'season'
        // Season
        // \s = whitespace
        // /d = one digit
        // () = captures this group so it can be accessed with ease later
        let regex = regex![r"Season\s(\d)"];

        let title = html
            .select(&title_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()[0];

        let season = regex
            .captures(title)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        return Some(season);
    }

    None
}

pub fn season_chapter(html: Option<&Html>, chapter: u16) -> Option<u16> {
    // input eg. '[Season 3] Ep. 133'
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();

        // Ep. (133) where 133 is captured
        // Ep.
        // \s = whitespace
        // /d+ = one or more digits
        // () = captures this group so it can be accessed with ease later
        let regex = regex![r"Ep.\s(\d+)"];

        let title = html
            .select(&title_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()[0];

        let season_chapter = regex
            .captures(title)?
            .get(1)?
            .as_str()
            .parse::<u16>()
            .unwrap();

        return Some(season_chapter);
    }
    None
}

pub const fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    if let Some(html) = html {}
    None
}

pub const fn custom(html: Option<&Html>, chapter: u16) -> Option<String> {
    if let Some(html) = html {}
    None
}

#[cfg(test)]
mod tower_of_god_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_produce_season() {
        const SEASON_NUMBER: &str = r#"<div class="subj_info">
        <a href="https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95" class="subj NPI=a:end,g:en_en" title="Tower of God">Tower of God</a>
        <span class="ico_arr2"></span>
        <h1 class="subj_episode" title="[Season 3] Ep. 133">[Season 3] Ep. 133</h1>
    </div>"#;

        let html = Html::parse_document(SEASON_NUMBER);

        let season_number = season(Some(&html), 0).unwrap();

        assert_eq!(season_number, 3);
    }

    #[test]
    fn should_produce_season_chapter() {
        const SEASON_CHAPTER_NUMBER1: &str = r##"<div class="subj_info">
        <a href="https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95" class="subj NPI=a:end,g:en_en" title="Tower of God">Tower of God</a>
        <span class="ico_arr2"></span>
        <h1 class="subj_episode" title="[Season 3] Ep. 133">[Season 3] Ep. 133</h1>
    </div>"##;

        const SEASON_CHAPTER_NUMBER2: &str = r##"<div class="subj_info">
        <a href="https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95" class="subj NPI=a:end,g:en_en" title="Tower of God">Tower of God</a>
        <span class="ico_arr2"></span>
        <h1 class="subj_episode" title="[Season 1] Ep. 1 - 1F.Headon's Floor">[Season 1] Ep. 1 - 1F.Headon's Floor</h1>
    </div>"##;

        let html1 = Html::parse_document(SEASON_CHAPTER_NUMBER1);

        let html2 = Html::parse_document(SEASON_CHAPTER_NUMBER2);

        let result1 = season_chapter(Some(&html1), 0).unwrap();

        let result2 = season_chapter(Some(&html2), 0).unwrap();

        assert_eq!(result1, 133);
        assert_eq!(result2, 1);
    }

    #[test]
    #[ignore]
    fn should_produce_arc() {}

    #[test]
    #[ignore]
    fn should_produce_custom() {}
}
