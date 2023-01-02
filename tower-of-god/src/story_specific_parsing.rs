use project_core::regex;
use scraper::{Html, Selector};

// Story specific parsing implementations go here.

pub fn parse_season_number(html: &Html) -> u8 {
    // input eg. '[Season 3] Ep. 133'

    let title_selector = Selector::parse("h1.subj_episode").unwrap();
    // Season (3) where 3 is captured as 'season'
    // Season
    // \s = whitespace
    // /d = one digit
    // () = captures this group so it can be accessed with ease later
    let regex = regex![r"Season\s(\d)"];

    let title = html
        .select(&title_selector)
        .into_iter()
        .next()
        .unwrap()
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

    season
}

pub fn parse_season_chapter_number(html: &Html) -> u16 {
    // input eg. '[Season 3] Ep. 133'

    let title_selector = Selector::parse("h1.subj_episode").unwrap();

    // Ep. (133) where 133 is captured
    // Ep.
    // \s = whitespace
    // /d+ = one or more digits
    // () = captures this group so it can be accessed with ease later
    let regex = regex![r"Ep.\s(\d+)"];

    let title = html
        .select(&title_selector)
        .into_iter()
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    let season_chapter = regex
        .captures(title)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap();

    season_chapter
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_season_number() {
        const SEASON_NUMBER: &str = r#"<div class="subj_info">
        <a href="https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95" class="subj NPI=a:end,g:en_en" title="Tower of God">Tower of God</a>
        <span class="ico_arr2"></span>
        <h1 class="subj_episode" title="[Season 3] Ep. 133">[Season 3] Ep. 133</h1>
    </div>"#;

        let html = Html::parse_document(SEASON_NUMBER);

        let season_number = parse_season_number(&html);

        assert_eq!(season_number, 3);
    }

    #[test]
    fn should_parse_season_chapter_number() {
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

        let result1 = parse_season_chapter_number(&html1);

        let result2 = parse_season_chapter_number(&html2);

        assert_eq!(result1, 133);
        assert_eq!(result2, 1);
    }
}
