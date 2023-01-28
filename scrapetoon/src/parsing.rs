#![allow(unused)]
use scraper::Html;

pub const fn season(html: Option<&Html>, chapter: u16) -> Option<u8> {
    None
}

pub const fn season_chapter(html: Option<&Html>, chapter: u16) -> Option<u16> {
    None
}

pub const fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    None
}

pub const fn custom(html: Option<&Html>, chapter: u16) -> Option<String> {
    None
}

#[cfg(test)]
mod scraper_tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    #[test]
    #[ignore]
    fn should_produce_custom() {
        todo!()
    }
}
