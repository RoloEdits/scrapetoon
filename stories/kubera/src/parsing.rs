#![allow(unused)]

use scraper::{Html, Selector};
use webtoons::regex;

pub fn season(html: Option<&Html>, chapter: u16) -> Option<u8> {
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();

        let regex = regex![r"Season\s(\d)"];

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
        .expect("Failed to parse season from title");

        return Some(season);
    }

    None
}

pub fn season_chapter(html: Option<&Html>, chapter: u16) -> Option<u16> {
    if let Some(html) = html {
        let title_selector = Selector::parse("h1.subj_episode").unwrap();

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

pub fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    if let Some(html) = html {
        let arc_title_selector = Selector::parse("h1.subj_episode").unwrap();

        // TODO: Figure out why last character is being removed from capture group
        let regex = regex![r"-([A-Za-z'\s]+)"];

        let title = html
            .select(&arc_title_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()[0];

        let arc_title = regex.captures(title)?.get(1)?.as_str();

        return Some(arc_title.trim().to_string());
    }

    None
}

pub const fn custom(html: Option<&Html>, chapter: u16) -> Option<String> {
    if let Some(html) = html {}
    None
}

#[cfg(test)]
mod kubera_tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_produce_season_number() {
        const SEASON_NUMBER1: &str = r#"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Ep. 50 -  Half (半) (8)">Ep. 50 -  Half (半) (8)</h1>
					</div>"#;

        const SEASON_NUMBER2: &str = r#"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="[Season 2] Ep. 43 - Rift (3)">[Season 2] Ep. 43 - Rift (3)</h1>
					</div>"#;

        const SEASON_NUMBER3: &str = r#"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="[Season 3] Ep. 165 - The Weight of Time (5)">[Season 3] Ep. 165 - The Weight of Time (5)</h1>
					</div>"#;

        let html1 = Html::parse_document(SEASON_NUMBER1);
        let html2 = Html::parse_document(SEASON_NUMBER2);
        let html3 = Html::parse_document(SEASON_NUMBER3);

        let season_number1 = season(Some(&html1), 0).unwrap();
        let season_number2 = season(Some(&html2), 0).unwrap();
        let season_number3 = season(Some(&html3), 0).unwrap();

        assert_eq!(season_number1, 1);
        assert_eq!(season_number2, 2);
        assert_eq!(season_number3, 3);
    }

    #[test]
    fn should_produce_season_chapter_number() {
        const SEASON_CHAPTER_NUMBER1: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Ep. 50 -  Half (半) (8)">Ep. 50 -  Half (半) (8)</h1>
					</div>"##;

        const SEASON_CHAPTER_NUMBER2: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="[Season 3] Ep. 165 - The Weight of Time (5)">[Season 3] Ep. 165 - The Weight of Time (5)</h1>
					</div>"##;

        let html1 = Html::parse_document(SEASON_CHAPTER_NUMBER1);

        let html2 = Html::parse_document(SEASON_CHAPTER_NUMBER2);

        let result1 = season_chapter(Some(&html1), 0).unwrap();

        let result2 = season_chapter(Some(&html2), 0).unwrap();

        assert_eq!(result1, 50);
        assert_eq!(result2, 165);
    }

    #[test]
    fn should_produce_arc() {
        const ARC_TITLE1: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Ep. 0 - Prologue">Ep. 0 - Prologue</h1>
					</div>"##;

        const ARC_TITLE2: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/fantasy/kubera/list?title_no=83" class="subj NPI=a:end,g:en_en" title="Kubera">Kubera</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Ep. 1 - A Girl With a God's Name (1)">Ep. 1 - A Girl With a God's Name (1)</h1>
					</div>"##;

        let html1 = Html::parse_document(ARC_TITLE1);

        let html2 = Html::parse_document(ARC_TITLE2);

        let result1 = arc(Some(&html1), 0).unwrap();

        let result2 = arc(Some(&html2), 0).unwrap();

        assert_eq!(result1, "Prologue");
        assert_eq!(result2, "A Girl With a God's Name");
    }

    #[test]
    #[ignore]
    fn should_produce_custom() {
        todo!()
    }
}
