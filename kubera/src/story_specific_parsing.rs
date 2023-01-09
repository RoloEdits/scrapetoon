use project_core::regex;
use scraper::{Html, Selector};

pub fn parse_season_number(html: &Html) -> u8 {
    let title_selector = Selector::parse("h1.subj_episode").unwrap();

    let regex = regex![r"Season\s(\d)"];

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

pub fn parse_season_chapter_number(html: &Html) -> u16 {
    let title_selector = Selector::parse("h1.subj_episode").unwrap();

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

pub fn parse_arc_title(html: &Html) -> String {
    let arc_title_selector = Selector::parse("h1.subj_episode").unwrap();

    // TODO: Figure out why last character is being removed from capture group
    let regex = regex![r"-([A-Za-z'\s]+)"];

    let title = html
        .select(&arc_title_selector)
        .into_iter()
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    let arc_title = regex.captures(title).unwrap().get(1).unwrap().as_str();

    arc_title.trim().to_string()
}

// pub const fn chapter_number_correction(chapter_number: u16) -> u16 {
//     if chapter_number > 285 {
//         return chapter_number - 4;
//     }
//
//     if chapter_number > 267 {
//         return chapter_number - 2;
//     }
//
//     if chapter_number > 102 {
//         return chapter_number - 1;
//     }
//
//     chapter_number
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_season_number() {
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

        let season_number1 = parse_season_number(&html1);
        let season_number2 = parse_season_number(&html2);
        let season_number3 = parse_season_number(&html3);

        assert_eq!(season_number1, 1);
        assert_eq!(season_number2, 2);
        assert_eq!(season_number3, 3);
    }

    #[test]
    fn should_parse_season_chapter_number() {
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

        let result1 = parse_season_chapter_number(&html1);

        let result2 = parse_season_chapter_number(&html2);

        assert_eq!(result1, 50);
        assert_eq!(result2, 165);
    }

    #[test]
    fn should_parse_arc_title() {
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

        let result1 = parse_arc_title(&html1);

        let result2 = parse_arc_title(&html2);

        assert_eq!(result1, "Prologue");
        assert_eq!(result2, "A Girl With a God's Name");
    }

    #[test]
    fn should_correct_chapter_number() {
        let result1 = chapter_number_correction(100);
        let result2 = chapter_number_correction(110);
        let result3 = chapter_number_correction(270);
        let result4 = chapter_number_correction(300);

        assert_eq!(result1, 100);
        assert_eq!(result2, 109);
        assert_eq!(result3, 268);
        assert_eq!(result4, 296);
    }
}
