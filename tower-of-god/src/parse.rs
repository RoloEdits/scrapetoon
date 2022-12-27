use project_core::regex;
use scraper::{Html, Selector};

pub fn parse_season_number(html: &Html) -> u8 {
    let season_selector = Selector::parse("h1.subj_episode").unwrap();

    let mut result: u8 = 0;

    let regex = regex![r"\d]"];

    for element in html.select(&season_selector) {
        let number = element.text().collect::<Vec<_>>()[0];

        let season_number = regex.captures(number).unwrap()[0].to_string();

        result = season_number
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap_or_else(|| panic!("Error parsing: [{}]", number)) as u8;
    }

    result
}

pub fn parse_season_chapter_number(html: &Html) -> u16 {
    let season_chapter_number_selector = Selector::parse("h1.subj_episode").unwrap();

    let chapter_number = html
        .select(&season_chapter_number_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    chapter_number[15..]
        .parse::<u16>()
        // solves Ep. 1 - Headons floor where others are all Ep. 2, Ep.3, etc
        .unwrap_or(1)
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

        let html1 = Html::parse_document(SEASON_CHAPTER_NUMBER1);

        const SEASON_CHAPTER_NUMBER2: &str = r##"<div class="subj_info">
        <a href="https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95" class="subj NPI=a:end,g:en_en" title="Tower of God">Tower of God</a>
        <span class="ico_arr2"></span>
        <h1 class="subj_episode" title="[Season 1] Ep. 1 - 1F.Headon's Floor">[Season 1] Ep. 1 - 1F.Headon's Floor</h1>
    </div>"##;

        let html2 = Html::parse_document(SEASON_CHAPTER_NUMBER2);

        let result1 = parse_season_chapter_number(&html1);

        let result2 = parse_season_chapter_number(&html2);

        assert_eq!(result1, 133);
        assert_eq!(result2, 1);
    }
}
