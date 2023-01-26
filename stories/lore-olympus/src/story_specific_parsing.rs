use project_core::regex;
use scraper::{Html, Selector};

pub fn parse_season_number(html: &Html) -> u8 {
    let title_selector = Selector::parse("h1.subj_episode").unwrap();

    let regex = regex![r"\(S(\d)\)"];

    let title = html
        .select(&title_selector)
        .into_iter()
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    let season = match regex.captures(title) {
        Some(cap) => cap,
        None => return 1,
    }
    .get(1)
    .unwrap()
    .as_str()
    .parse::<u8>()
    .unwrap();

    season
}

pub fn parse_meaningful_chapter_number(html: &Html) -> u16 {
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

    meaningful_chapter_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_season_number() {
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

        let season_number = parse_season_number(&html);
        let season_number2 = parse_season_number(&html2);

        assert_eq!(season_number, 3);
        assert_eq!(season_number2, 1);
    }

    #[test]
    fn should_parse_meaningful_chapter_number() {
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

        let result1 = parse_meaningful_chapter_number(&html1);
        let result2 = parse_meaningful_chapter_number(&html2);

        assert_eq!(result1, 1);
        assert_eq!(result2, 116);
    }
}
