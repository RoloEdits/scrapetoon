use project_core::regex;
use scraper::{Html, Selector};

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
    fn should_parse_meaningful_chapter_number() {
        const CHAPTER_NUMBER1: &str = r##"<div class="subj_info">
						<a href="https://www.webtoons.com/en/super-hero/unordinary/list?title_no=679" class="subj NPI=a:end,g:en_en" title="unOrdinary">unOrdinary</a>
						<span class="ico_arr2"></span>
						<h1 class="subj_episode" title="Episode 78">Episode 78</h1>
					</div>"##;

        let html1 = Html::parse_document(CHAPTER_NUMBER1);

        let result1 = parse_meaningful_chapter_number(&html1);

        assert_eq!(result1, 78);
    }
}
