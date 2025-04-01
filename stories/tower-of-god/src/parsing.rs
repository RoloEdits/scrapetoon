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

pub fn arc(html: Option<&Html>, chapter: u16) -> Option<String> {
    match chapter {
        // Season 1
        // The Tower
        1..=4 => Some("Headon's Floor".to_string()),
        5..=8 => Some("Evankhell's Hell".to_string()),
        9..=10 => Some("Lero-Ro's Test".to_string()),
        11..=13 => Some("Yu Han Sung's Examination".to_string()),
        14..=25 => Some("Crown Game".to_string()),
        26..=27 => Some("Rest".to_string()),
        28..=30 => Some("Position Test".to_string()),
        31..=34 => Some("Zahard's Princess".to_string()),
        35..=51 => Some("Hide-and-Seek".to_string()),
        52..=56 => Some("Submerged Fish".to_string()),
        57..=74 => Some("Last Examination".to_string()),
        75..=78 => Some("Rachel".to_string()),
        // Season 2
        // The Prince of Zahard
        79..=80 => Some("Season 2 - Prologue".to_string()),
        81..=85 => Some("Last Chance".to_string()),
        86..=88 => Some("The Strongest Regular".to_string()),
        89..=90 => Some("Bath".to_string()),
        91..=98 => Some("The Untrustworthy Room".to_string()),
        99..=101 => Some("The Preys".to_string()),
        102 => Some("Epilogue".to_string()),
        103..=105 => Some("The Wool's Knot".to_string()),
        106..=108 => Some("FUG".to_string()),
        109..=113 => Some("Zygaena's Flower".to_string()),
        114..=115 => Some("The Way".to_string()),
        // Extra Floor
        116..=117 => Some("Extra Floor".to_string()),
        // The Workshop Battle
        118 => Some("Emile".to_string()),
        // The Hand of Arlene
        119..=121 => Some("Connection".to_string()),
        122..=131 => Some("Devil of the Right Arm".to_string()),
        132..=135 => Some("Bet".to_string()),
        136..=137 => Some("Workshop Battle".to_string()),
        138..=145 => Some("One Shot, One Opportunity".to_string()),
        146..=148 => Some("Archimedes".to_string()),
        149..=156 => Some("Battle x Gamble".to_string()),
        157..=160 => Some("The Truth".to_string()),
        161..=164 => Some("Tournament".to_string()),
        165..=170 => Some("Thorn".to_string()),
        171..=174 => Some("The Summoning".to_string()),
        175..=187 => Some("Closure".to_string()),
        188..=190 => Some("The Workshop Battle: Epilogue".to_string()),
        // The Hell Train
        // Train City
        191..=192 => Some("Prologue".to_string()),
        193..=233 => Some("Revolution Road".to_string()),
        // The Dallar Show
        234..=240 => Some("Hoaqin".to_string()),
        241..=246 => Some("Wooden Horse".to_string()),
        247..=252 => Some("A Month".to_string()),
        253..=276 => Some("The Dallar Show".to_string()),
        // The Name Hunt Station
        277..=278 => Some("Yuri Zahard".to_string()),
        279..=306 => Some("The 'Name Hunt' Station".to_string()),
        307..=312 => Some("Wangnan".to_string()),
        // The Floor of Death
        313..=340 => Some("The Floor of Death".to_string()),
        341..=344 => Some("New Power".to_string()),
        // The Hidden Floor
        345..=355 => Some("The Hidden Floor".to_string()),
        356..=358 => Some("The Hidden Hidden Floor".to_string()),
        359..=364 => Some("Khun Eduan".to_string()),
        365..=368 => Some("Zahard's Data".to_string()),
        369..=378 => Some("Training".to_string()),
        379..=382 => Some("Tomorrow".to_string()),
        383..=386 => Some("Power".to_string()),
        387..=389 => Some("The Advent".to_string()),
        390..=396 => Some("Three Orders".to_string()),
        // The Last Station
        397..=403 => Some("The Last Station".to_string()),
        404..=407 => Some("Evankhell".to_string()),
        408..=416 => Some("Kallavan".to_string()),
        417..=418 => Some("New Wave".to_string()),
        // Season 3
        // The Nest
        // The Cage
        419..=425 => Some("Deng Deng".to_string()),
        426..=427 => Some("Baylord Yama".to_string()),
        428..=434 => Some("Stealing the Fang".to_string()),
        435..=437 => Some("Heart".to_string()),
        438..=439 => Some("Baylord Doom".to_string()),
        440..=441 => Some("Stealing the Fang 2".to_string()),
        442..=444 => Some("King of the Dogs".to_string()),
        445..=447 => Some("Khel Hellam".to_string()),
        448..=450 => Some("Transformation".to_string()),
        // The Wall of Peaceful Coexistence
        451..=455 => Some("The Wall of Peaceful Coexistence".to_string()),
        456..=460 => Some("The Wall with a Sleeping Forget-Me-Not".to_string()),
        461..=462 => Some("Kallavan vs White".to_string()),
        463..=466 => Some("Dowon".to_string()),
        // The Nest
        467..=470 => Some("The Nest".to_string()),
        471..=473 => Some("The Intrusion".to_string()),
        474..=477 => Some("A Rough War".to_string()),
        478..=484 => Some("VS. Kallavan".to_string()),
        485..=487 => Some("The Second Defensive Wall".to_string()),
        488..=492 => Some("Gakjadosaeng".to_string()),
        493..=499 => Some("All Out Fight".to_string()),
        500..=514 => Some("A Dark Change".to_string()),
        515..=519 => Some("The One High Above".to_string()),
        520..=524 => Some("Warp Gate".to_string()),
        525..=526 => Some("The Descent".to_string()),
        527..=530 => Some("A Dog And A Cat".to_string()),
        531..=536 => Some("Defensive Battle".to_string()),
        537..=539 => Some("Turning The Tide".to_string()),
        540..=541 => Some("The Offer".to_string()),
        542..=545 => Some("The Ten Great Family Leaders (Calamity)".to_string()),
        546..=547 => Some("Warp".to_string()),
        548.. => Some("Lilia Zahard".to_string()),
        _ => None,
    }
}

pub fn custom(html: Option<&Html>, chapter: u16) -> Option<String> {
    // Saga's
    match chapter {
        1..=78 => Some("The Tower".to_string()),
        79..=117 => Some("The Prince of Zahard".to_string()),
        118..=190 => Some("The Workshop Battle".to_string()),
        191..=417 => Some("The Hell Train".to_string()),
        418.. => Some("The Nest".to_string()),
        _ => None,
    }
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
