pub mod chapter;
pub mod chapter_list;
pub mod models;

use crate::factories::BlockingReferClient;
use crate::{regex, Arc, Season, SeasonChapter, SkipChapter};
use anyhow::{anyhow, bail, Context, Result};
use core::time;
use models::{Story, StoryPage};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::thread;

const ONGOING: &str = "Ongoing";

// Just one over the limit and, for now, it is easier to follow by having explicit types and names in the argument list
#[allow(clippy::too_many_arguments)]
/// # Errors
pub fn parse(
    start: u16,
    end: u16,
    url: &str,
    season: Season,
    season_chapter: SeasonChapter,
    arc: Arc,
    skip: SkipChapter,
    is_completed: bool,
    chapter_published: Option<&HashMap<u16, String>>,
) -> Result<(Story, String)> {
    let (id, kebab_title) = parse_url(url);
    let story_page = story_page(url)?;
    let chapters = chapter::parse(
        start,
        end,
        id,
        season,
        season_chapter,
        arc,
        skip,
        is_completed,
        chapter_published,
    )?;

    Ok((
        Story {
            story_page,
            chapters,
        },
        kebab_title,
    ))
}

// Series Page
fn story_page(url: &str) -> Result<StoryPage> {
    let html = BlockingReferClient::get(url)?
        .text()
        .context("Failed to get HTML body as text")?;

    let genre = genre(&html)?;
    let title = title(&html)?;
    let author = author(&html)?;
    let (release_day, status) = release_day_and_status(&html)?;
    let views = views(&html)?;
    let subscribers = subscribers(&html)?;
    let rating = rating(&html)?;

    thread::sleep(time::Duration::from_secs(3));

    Ok(StoryPage {
        title,
        author,
        genre,
        status,
        release_day,
        views,
        subscribers,
        rating,
    })
}

fn title(html: &str) -> Result<String> {
    let html = Html::parse_document(html);
    let title_selector = Selector::parse(r"h1.subj").expect("Failed to parse Title selector");

    if let Some(title_fragment) = html.select(&title_selector).next() {
        let title_text = title_fragment.text().collect::<Vec<_>>();
        let mut result = String::new();

        for word in title_text {
            result.push_str(word);
        }

        return Ok(result.replace(':', ": "));
    }

    bail!("Failed to parse title")
}

fn genre(html: &str) -> Result<String> {
    let html = Html::parse_document(html);
    let genre_selector = Selector::parse(r"h2.genre").expect("Failed to parse Genre Selector");

    let genre = html
        .select(&genre_selector)
        .next()
        .ok_or_else(|| anyhow!("Failed to parse genre element"))?
        .text()
        .next()
        .ok_or_else(|| anyhow!("Failed to parse genre to text"))?
        .to_string();

    Ok(genre)
}

fn rating(html: &str) -> Result<f32> {
    let html = Html::parse_document(html);
    let rating_selector =
        Selector::parse(r"em#_starScoreAverage").expect("Failed to parse rating selector");
    if let Some(rating_fragment) = html.select(&rating_selector).next() {
        let rating_text = rating_fragment
            .text()
            .next()
            .ok_or_else(|| anyhow!("Failed to parse rating to text"))?;

        let result = rating_text
            .parse::<f32>()
            .with_context(|| format!("Failed to parse {rating_text} as a f32"))?;
        return Ok(result);
    }

    bail!("Failed to parse rating")
}

fn subscribers(html: &str) -> Result<u32> {
    let html = Html::parse_document(html);
    let subscribers_selector =
        Selector::parse(r"em.cnt").expect("Failed to parse subscriber selector");

    if let Some(subs) = html.select(&subscribers_selector).nth(1) {
        if let Some(sub) = subs.text().collect::<Vec<_>>().first() {
            let result = match sub {
                m if m.ends_with('M') => {
                    let cleaned_m = m.replace('M', "");
                    let million = cleaned_m.parse::<f32>().with_context(|| {
                        format!("Failed to parse subscriber count. Value = {cleaned_m}")
                    })? * 1_000_000.0;
                    million as u32
                }
                k => {
                    let cleaned_k = k.replace(',', "");
                    cleaned_k.parse::<u32>().with_context(|| {
                        format!("Failed to parse view count. Value = {cleaned_k}")
                    })?
                }
            };

            return Ok(result);
        }
    }

    bail!("Failed to parse subscribers")
}

fn views(html: &str) -> Result<u64> {
    let html = Html::parse_document(html);
    let views_selector = Selector::parse(r"em.cnt").expect("Failed to create views selector");

    if let Some(views) = html.select(&views_selector).next() {
        let result =
            match *views
                .text()
                .collect::<Vec<_>>()
                .first()
                .ok_or_else(|| anyhow!(""))?
            {
                m if m.ends_with('M') => {
                    let cleaned_m = m.replace('M', "");
                    let million = cleaned_m.parse::<f64>().with_context(|| {
                        format!("Failed to parse view count. Value = {cleaned_m}")
                    })? * 1_000_000.0;
                    million as u64
                }
                b if b.ends_with('B') => {
                    let cleaned_b = b.replace('B', "");
                    let billion = cleaned_b.parse::<f64>().with_context(|| {
                        format!("Failed to parse view count. Value = {cleaned_b}")
                    })? * 1_000_000_000.0;
                    billion as u64
                }
                k => {
                    let cleaned_k = k.replace(',', "");
                    cleaned_k.parse::<u64>().with_context(|| {
                        format!("Failed to parse view count. Value = {cleaned_k}")
                    })?
                }
            };

        return Ok(result);
    }

    bail!("Failed to parse views")
}

fn release_day_and_status(html: &str) -> Result<(String, String)> {
    let html = Html::parse_document(&html.replace("EVERY ", "").replace("UP", ""));

    if let Ok(day_selector) = Selector::parse(r"p.day_info") {
        if let Some(day) = html.select(&day_selector).next() {
            // TODO: Make Day a Vec so stories with more than one day can show as such.
            let (day, status) = match *day
                .text()
                .collect::<Vec<_>>()
                .first()
                .ok_or_else(|| anyhow!("Day was Empty"))?
            {
                sun if sun.starts_with("SUN") => ("Sunday".to_string(), ONGOING.to_string()),
                mon if mon.starts_with("MON") => ("Monday".to_string(), ONGOING.to_string()),
                tue if tue.starts_with("TUE") => ("Tuesday".to_string(), ONGOING.to_string()),
                wed if wed.starts_with("WED") => ("Wednesday".to_string(), ONGOING.to_string()),
                thu if thu.starts_with("THU") => ("Thursday".to_string(), ONGOING.to_string()),
                fri if fri.starts_with("FRI") => ("Friday".to_string(), ONGOING.to_string()),
                sat if sat.starts_with("SAT") => ("Saturday".to_string(), ONGOING.to_string()),
                _ => ("Completed".to_string(), "Completed".to_string()),
            };

            return Ok((day, status));
        }

        bail!("Failed to parse a day");
    }

    bail!("Failed to create Day Selector")
}

fn author(html: &str) -> Result<String> {
    let html = Html::parse_document(html);

    if let Ok(author_with_link_selector) = Selector::parse(r"a.author") {
        if let Some(author_fragment) = html.select(&author_with_link_selector).next() {
            let author_text = author_fragment
                .text()
                .next()
                .ok_or_else(|| anyhow!("Failed to parse Author name"))?;

            let mut result = String::new();

            // This is for cases where there are, for some reason, webtoon putting a bunch of tabs and new-lines in the name.
            // 66,666 Years: Advent of the Dark Mage is the first example, unknown if the only.
            for text in author_text.trim().replace('\n', "").split_whitespace() {
                result.push_str(text);
                result.push(' ');
            }

            return Ok(result.trim().to_string());
        }
    }

    if let Ok(author_without_link_selector) = Selector::parse(r"div.author_area") {
        if let Some(author_fragment) = html.select(&author_without_link_selector).next() {
            let author_text = author_fragment
                .text()
                .next()
                .ok_or_else(|| anyhow!("Failed to parse Author name"))?;

            let mut result = String::new();

            // This is for cases where there are, for some reason, webtoon putting a bunch of tabs and new-lines in the name.
            // 66,666 Years: Advent of the Dark Mage is the first example, unknown if the only.
            for text in author_text.trim().replace('\n', "").split_whitespace() {
                result.push_str(text);
                result.push(' ');
            }

            return Ok(result.trim().to_string());
        }
    }

    bail!("Failed to create Author Selector")
}

// Series Helpers
fn parse_url(url: &str) -> (u32, String) {
    let reg = regex![
        r"https://www.webtoons.com/../(?P<genre>.+)/(?P<title>.+)/list\?title_no=(?P<id>\d+)"
    ];

    let cap = reg.captures(url).unwrap();

    let id = cap["id"].parse::<u32>().expect("Failed to get Id from URL");
    let kebab_title = cap["title"].to_string();

    (id, kebab_title)
}

#[cfg(test)]
mod series_info_parsing_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_rating_936() {
        const RATING: &str = r##"<ul class="grade_area">
        <li>
            <span class="ico_view">view</span>
            <em class="cnt">1B</em>
        </li>
        <li>
            <span class="ico_subscribe">subscribe</span>
            <em class="cnt">3.3M</em>
        </li>
        <li>
            <span class="ico_grade5">grade</span>
            <em class="cnt" id="_starScoreAverage">9.86</em>
            <a href="#" id="_rateButton" class="btn_rate2 NPI=a:rate,g:en_en" onclick="return false;">RATE</a>

            <div class="ly_grade">

                <strong id="_starScoreCount" class="grade_cnt">10<span class="blind">point</span></strong>

                <span class="star_area NPI=a:ratesel,g:en_en">
                    <a href="#" title="0">Selected points</a>
                    <a href="#" title="1" onclick="return false;" class="on _star">1 points</a>
                    <a href="#" title="2" onclick="return false;" class="st_r on _star">2 points</a>
                    <a href="#" title="3" onclick="return false;" class="on _star">3 points</a>
                    <a href="#" title="4" onclick="return false;" class="st_r on _star">4 points</a>
                    <a href="#" title="5" onclick="return false;" class="on _star">5 points</a>
                    <a href="#" title="6" onclick="return false;" class="st_r on _star">6 points</a>
                    <a href="#" title="7" onclick="return false;" class="on _star">7 points</a>
                    <a href="#" title="8" onclick="return false;" class="st_r on _star">8 points</a>
                    <a href="#" title="9" onclick="return false;" class="_star">Select 9 points</a>
                    <a href="#" title="10" onclick="return false;" class="st_r  _star">10 point</a>

                </span>
                <p class="grade_txt">[Rate] Click to rate this</p>
                <div class="grade_btn">
                    <a href="#" title="Cancel" class="lnk_cncl NPI=a:racancel,g:en_en" onclick="return false;">Cancel</a>
                    <a href="#" title="Send" class="lnk_send NPI=a:rasend,g:en_en" onclick="return false;">Send</a>
                </div>
            </div>

            <div class="ly_grade retry">
                <p class="grade_txt">You've already rated this.<br>Would you like to rate it again?</p>
                <div class="grade_btn">
                    <a href="#" title="No" class="lnk_cncl">No</a>
                    <a href="#" title="Yes" class="lnk_send">Yes</a>
                </div>
            </div>
        </li>
    </ul>"##;

        let result = rating(RATING).unwrap();

        assert_eq!(result, 9.86_f32);
    }

    #[test]
    fn should_parse_subscribers() {
        const SUBSCRIBERS_1: &str = r##"<ul class="grade_area">
        <li>
            <span class="ico_view">view</span>
            <em class="cnt">1B</em>
        </li>
        <li>
            <span class="ico_subscribe">subscribe</span>
            <em class="cnt">3.3M</em>
        </li>
        <li>
            <span class="ico_grade5">grade</span>
            <em class="cnt" id="_starScoreAverage">9.86</em>
            <a href="#" id="_rateButton" class="btn_rate2 NPI=a:rate,g:en_en" onclick="return false;">RATE</a>

            <div class="ly_grade">

                <strong id="_starScoreCount" class="grade_cnt">10<span class="blind">point</span></strong>
                </span>
                <p class="grade_txt">[Rate] Click to rate this</p>
                <div class="grade_btn">
                    <a href="#" title="Cancel" class="lnk_cncl NPI=a:racancel,g:en_en" onclick="return false;">Cancel</a>
                    <a href="#" title="Send" class="lnk_send NPI=a:rasend,g:en_en" onclick="return false;">Send</a>
                </div>
            </div>

            <div class="ly_grade retry">
                <p class="grade_txt">You've already rated this.<br>Would you like to rate it again?</p>
                <div class="grade_btn">
                    <a href="#" title="No" class="lnk_cncl">No</a>
                    <a href="#" title="Yes" class="lnk_send">Yes</a>
                </div>
            </div>
        </li>
    </ul>"##;

        const SUBSCRIBERS_2: &str = r##"<ul class="grade_area">
    <li>
        <span class="ico_view">view</span>
        <em class="cnt">1B</em>
    </li>
    <li>
        <span class="ico_subscribe">subscribe</span>
        <em class="cnt">232,037</em>
    </li>
    <li>
        <span class="ico_grade5">grade</span>
        <em class="cnt" id="_starScoreAverage">9.86</em>
        <a href="#" id="_rateButton" class="btn_rate2 NPI=a:rate,g:en_en" onclick="return false;">RATE</a>

        <div class="ly_grade">

            <strong id="_starScoreCount" class="grade_cnt">10<span class="blind">point</span></strong>
            </span>
            <p class="grade_txt">[Rate] Click to rate this</p>
            <div class="grade_btn">
                <a href="#" title="Cancel" class="lnk_cncl NPI=a:racancel,g:en_en" onclick="return false;">Cancel</a>
                <a href="#" title="Send" class="lnk_send NPI=a:rasend,g:en_en" onclick="return false;">Send</a>
            </div>
        </div>

        <div class="ly_grade retry">
            <p class="grade_txt">You've already rated this.<br>Would you like to rate it again?</p>
            <div class="grade_btn">
                <a href="#" title="No" class="lnk_cncl">No</a>
                <a href="#" title="Yes" class="lnk_send">Yes</a>
            </div>
        </div>
    </li>
</ul>"##;

        let result_1 = subscribers(SUBSCRIBERS_1).unwrap();
        let result_2 = subscribers(SUBSCRIBERS_2).unwrap();

        assert_eq!(result_1, 3_300_000);
        assert_eq!(result_2, 232_037);
    }

    #[test]
    fn should_parse_views() {
        const VIEWS_1: &str = r##"<ul class="grade_area">
        <li>
            <span class="ico_view">view</span>
            <em class="cnt">1B</em>
        </li>
        <li>
            <span class="ico_subscribe">subscribe</span>
            <em class="cnt">1.1M</em>
        </li>
        <li>
            <span class="ico_grade5">grade</span>
            <em class="cnt" id="_starScoreAverage">9.86</em>
            <a href="#" id="_rateButton" class="btn_rate2 NPI=a:rate,g:en_en" onclick="return false;">RATE</a>

            <div class="ly_grade">

                <strong id="_starScoreCount" class="grade_cnt">10<span class="blind">point</span></strong>

                <span class="star_area NPI=a:ratesel,g:en_en">
                    <a href="#" title="0">Selected points</a>
                    <a href="#" title="1" onclick="return false;" class="on _star">1 points</a>
                    <a href="#" title="2" onclick="return false;" class="st_r on _star">2 points</a>
                    <a href="#" title="3" onclick="return false;" class="on _star">3 points</a>
                    <a href="#" title="4" onclick="return false;" class="st_r on _star">4 points</a>
                    <a href="#" title="5" onclick="return false;" class="on _star">5 points</a>
                    <a href="#" title="6" onclick="return false;" class="st_r on _star">6 points</a>
                    <a href="#" title="7" onclick="return false;" class="on _star">7 points</a>
                    <a href="#" title="8" onclick="return false;" class="st_r on _star">8 points</a>
                    <a href="#" title="9" onclick="return false;" class="_star">Select 9 points</a>
                    <a href="#" title="10" onclick="return false;" class="st_r  _star">10 point</a>

                </span>
                <p class="grade_txt">[Rate] Click to rate this</p>
                <div class="grade_btn">
                    <a href="#" title="Cancel" class="lnk_cncl NPI=a:racancel,g:en_en" onclick="return false;">Cancel</a>
                    <a href="#" title="Send" class="lnk_send NPI=a:rasend,g:en_en" onclick="return false;">Send</a>
                </div>
            </div>

            <div class="ly_grade retry">
                <p class="grade_txt">You've already rated this.<br>Would you like to rate it again?</p>
                <div class="grade_btn">
                    <a href="#" title="No" class="lnk_cncl">No</a>
                    <a href="#" title="Yes" class="lnk_send">Yes</a>
                </div>
            </div>
        </li>
    </ul>"##;

        const VIEWS_2: &str = r#"<li>
    <span class="ico_view">view</span>
    <em class="cnt">245,678</em>
</li>"#;

        const VIEWS_3: &str = r#"<li>
    <span class="ico_view">view</span>
    <em class="cnt">1.1M</em>
</li>"#;

        const VIEWS_4: &str = r#"<li>
    <span class="ico_view">view</span>
    <em class="cnt">956.3M</em>
</li>"#;

        let result_1 = views(VIEWS_1).unwrap();
        let result_2 = views(VIEWS_2).unwrap();
        let result_3 = views(VIEWS_3).unwrap();
        let result_4 = views(VIEWS_4).unwrap();

        assert_eq!(result_1, 1_000_000_000);
        assert_eq!(result_2, 245_678);
        assert_eq!(result_3, 1_100_000);
        assert_eq!(result_4, 956_300_000);
    }

    #[test]
    fn should_parse_release_day_and_is_completed() {
        const DAY: &str =
            r##"<p class="day_info"><span class="txt_ico_up">UP</span>EVERY MONDAY</p>"##;

        const COMPLETED: &str =
            r##"<p class="day_info"><span class="txt_ico_up">UP</span>COMPLETED</p>"##;

        let monday = release_day_and_status(DAY).unwrap();
        let completed = release_day_and_status(COMPLETED).unwrap();

        assert_eq!(monday, ("Monday".to_string(), "Ongoing".to_string()));
        assert_eq!(
            completed,
            ("Completed".to_string(), "Completed".to_string())
        );
    }

    #[test]
    fn should_parse_author_name() {
        const AUTHOR: &str = r##"<div class="author_area">
        <a href="https://www.webtoons.com/en/creator/instantmiso" class="author NPI=a:creator,g:en_en _gaLoggingLink">instantmiso</a>
<button type="button" class="ico_info2 _btnAuthorInfo">author info</button>
</div>"##;

        const AUTHOR_1: &str = r##"<div class="author_area">
        HYBE
<button type="button" class="ico_info2 _btnAuthorInfo">author info</button>
</div>"##;

        const AUTHOR_2: &str = r##"<div class="author_area">




        PASA


        ,


        TARU


        ...
        <button type="button" class="ico_info2 _btnAuthorInfo">author info</button>
        </div>"##;

        let author_1 = author(AUTHOR).unwrap();
        let author_2 = author(AUTHOR_1).unwrap();
        let author_3 = author(AUTHOR_2).unwrap();

        assert_eq!(author_1, "instantmiso".to_string());
        assert_eq!(author_2, "HYBE".to_string());
        assert_eq!(author_3, "PASA , TARU ...");
    }

    #[test]
    fn should_parse_series_title() {
        const TITLE: &str = r#"<div class="info">
        <h2 class="genre g_fantasy">Fantasy</h2>
        <h1 class="subj">DARK MOON:<br>THE BLOOD ALTAR</h1>
        <div class="author_area">
                    HYBE
            <button type="button" class="ico_info2 _btnAuthorInfo">author info</button>
        </div>
    </div>"#;

        let result = title(TITLE).unwrap();

        assert_eq!(result, "DARK MOON: THE BLOOD ALTAR");
    }

    #[test]
    fn should_parse_genre() {
        const GENRE: &str = r#"<div class="info">
						<h2 class="genre g_romance">Romance</h2>
						<h1 class="subj">Lore Olympus</h1>
						<div class="author_area">
										<a href="https://www.webtoons.com/en/creator/rachelsmythe" class="author NPI=a:creator,g:en_en _gaLoggingLink">Rachel Smythe</a>
							<button type="button" class="ico_info2 _btnAuthorInfo">author info</button>
						</div>
					</div>"#;

        let result = genre(GENRE).unwrap();

        assert_eq!(result, "Romance");
    }
}
