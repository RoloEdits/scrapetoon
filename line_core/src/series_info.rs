use core::time;
use project_core::ResponseFactory;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::thread;

use crate::{chapter_list, LikesDate};

use super::SeriesInfo;

#[must_use]
pub fn parse(end: u16, input_url: &str) -> SeriesInfo {
    let (title, author, status, release_day, views, subscribers, rating, genre) =
        parse_series_page_info(input_url);
    let chapter_list_info = chapter_list::parse(end, input_url);

    SeriesInfo {
        title,
        author,
        genre,
        status,
        release_day,
        views,
        subscribers,
        rating,
        chapter_list_info,
    }
}

#[must_use]
pub fn get_extra_info(pages: u16, url: &str) -> (SeriesInfo, HashMap<u16, LikesDate>) {
    println!("Pre-Fetching Necessary Data");
    let series_info = parse(pages, url);
    println!("Completed Pre-Fetch");

    let mut likes_date_hashmap: HashMap<u16, LikesDate> = HashMap::new();

    for chapter in &series_info.chapter_list_info {
        match likes_date_hashmap.insert(
            chapter.chapter_number,
            LikesDate::new(chapter.likes, chapter.date.clone()),
        ) {
            None | Some(_) => continue,
        };
    }

    (series_info, likes_date_hashmap)
}

// Series Page
#[tokio::main]
async fn parse_series_page_info(
    url: &str,
) -> (String, String, String, String, u64, u32, f32, String) {
    let html = ResponseFactory::get(url)
        .await
        .map_or_else(
            |_| panic!("Error connecting to URL webpage: {url}"),
            |html_response| html_response,
        )
        .text()
        .await
        .expect("Error getting HTML from response");

    let genre = parse_genre(&html);
    let title = parse_series_page_title(&html);
    let author = parse_series_page_author(&html);
    let (release_day, status) = parse_series_page_release_day_and_status(&html);
    let views = parse_series_page_views(&html);
    let subscribers = parse_series_page_subscribers(&html);
    let rating = parse_series_page_rating(&html);

    thread::sleep(time::Duration::from_secs(3));

    (
        title,
        author,
        status,
        release_day,
        views,
        subscribers,
        rating,
        genre,
    )
}

fn parse_series_page_title(html: &str) -> String {
    let html = Html::parse_document(html);
    let title_selector = Selector::parse(r"h1.subj").unwrap();

    let mut title_element = html.select(&title_selector);
    let title_fragment = title_element.next().unwrap();
    let title_text = title_fragment.text().collect::<Vec<_>>();

    let mut result = String::new();

    for word in title_text {
        result.push_str(word);
    }

    result.replace(':', ": ")
}

fn parse_genre(html: &str) -> String {
    let html = Html::parse_document(html);
    let genre_selector = Selector::parse(r"h2.genre").unwrap();

    let genre = html
        .select(&genre_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

    genre
}

fn parse_series_page_rating(html: &str) -> f32 {
    let html = Html::parse_document(html);
    let rating_selector = Selector::parse(r"em#_starScoreAverage").unwrap();

    let mut rating_element = html.select(&rating_selector);
    let rating_fragment = rating_element.next().unwrap();
    let rating_text = rating_fragment.text().next().unwrap();

    rating_text.parse::<f32>().unwrap()
}

fn parse_series_page_subscribers(html: &str) -> u32 {
    let html = Html::parse_document(html);
    let subscribers_selector = Selector::parse(r"em.cnt").unwrap();

    let subscribers_element = html.select(&subscribers_selector);

    let mut result: String = String::new();

    for (iteration, element) in subscribers_element.enumerate() {
        if iteration == 1 {
            result = element.text().collect::<Vec<_>>()[0].to_string();
            break;
        }
    }

    match result {
        sub_text if sub_text.ends_with('M') => {
            let float = sub_text
                .replace('M', "")
                .parse::<f32>()
                .unwrap_or_else(|_| {
                    panic!("Error! Couldn't get subscriber count. Value ={sub_text}")
                })
                * 1_000_000.0;
            float as u32
        }
        sub_text => sub_text
            .replace(',', "")
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Error! Couldn't get subscriber count. Value ={sub_text}")),
    }
}

fn parse_series_page_views(html: &str) -> u64 {
    let html = Html::parse_document(html);
    let views_selector = Selector::parse(r"em.cnt").unwrap();

    let views_element = html.select(&views_selector);

    let mut result: String = String::new();

    for (iteration, element) in views_element.enumerate() {
        if iteration == 0 {
            result = element.text().collect::<Vec<_>>()[0].to_string();
            break;
        }
    }

    match result {
        sub_text if sub_text.ends_with('M') => {
            let million = sub_text
                .replace('M', "")
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={sub_text}"))
                * 1_000_000.0;
            million as u64
        }
        sub_text if sub_text.ends_with('B') => {
            let billion = sub_text
                .replace('B', "")
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={sub_text}"))
                * 1_000_000_000.0;
            billion as u64
        }
        sub_text => sub_text
            .replace(',', "")
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={sub_text}")),
    }
}

fn parse_series_page_release_day_and_status(html: &str) -> (String, String) {
    let html = Html::parse_document(&html.replace("EVERY ", "").replace("UP", ""));
    let day_selector = Selector::parse(r"p.day_info").unwrap();

    let day_element = html.select(&day_selector);

    let mut result: String = String::new();

    for (iteration, element) in day_element.enumerate() {
        if iteration == 0 {
            result = element.text().collect::<Vec<_>>()[0].to_string();
            break;
        }
    }

    const ONGOING: &str = "Ongoing";

    // TODO: Make Day a Vec so stories with more than one day can show as such.
    let (day, status) = match result {
        sub_text if sub_text.starts_with("SUN") => ("Sunday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("MON") => ("Monday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("TUE") => ("Tuesday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("WED") => ("Wednesday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("THU") => ("Thursday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("FRI") => ("Friday".to_string(), ONGOING.to_string()),
        sub_text if sub_text.starts_with("SAT") => ("Saturday".to_string(), ONGOING.to_string()),
        _ => ("Completed".to_string(), "Completed".to_string()),
    };

    (day, status)
}

fn parse_series_page_author(html: &str) -> String {
    let html = Html::parse_document(html);
    let author_with_link_selector = Selector::parse(r"a.author").unwrap();
    let author_without_link_selector = Selector::parse(r"div.author_area").unwrap();

    let mut author_element = html.select(&author_with_link_selector);
    let author_fragment: ElementRef = author_element.next().map_or_else(
        || {
            let without_link = html.select(&author_without_link_selector).next().unwrap();

            without_link
        },
        |some| some,
    );

    let author_text = author_fragment.text().next().unwrap();

    let mut result = String::new();

    // This is for cases where there are, for some reason, webtoon putting a bunch of tabs and new-lines in the name.
    // 66,666 Years: Advent of the Dark Mage is the first example, unknown if the only.
    for text in author_text.trim().replace('\n', "").split_whitespace() {
        result.push_str(text);
        result.push(' ');
    }

    result.trim().to_string()
}

// Series Helpers
// fn parse_url_info(url: &str) -> (String, u16) {
//     let reg = regex![
//         r"https://www.webtoons.com/../(?P<genre>.+)/(?P<title>.+)/list\?title_no=(?P<id>\d+)"
//     ];
//
//     let cap = reg.captures(url).unwrap();
//
//     (cap["genre"].to_string(), cap["id"].parse::<u16>().unwrap())
// }

#[cfg(test)]
mod series_info_parsing_tests {
    use super::*;

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

        let result = parse_series_page_rating(RATING);

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

        let result_1 = parse_series_page_subscribers(SUBSCRIBERS_1);
        let result_2 = parse_series_page_subscribers(SUBSCRIBERS_2);

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

        let result_1 = parse_series_page_views(VIEWS_1);
        let result_2 = parse_series_page_views(VIEWS_2);
        let result_3 = parse_series_page_views(VIEWS_3);
        let result_4 = parse_series_page_views(VIEWS_4);

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

        let monday = parse_series_page_release_day_and_status(DAY);
        let completed = parse_series_page_release_day_and_status(COMPLETED);

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

        let author = parse_series_page_author(AUTHOR);
        let author_1 = parse_series_page_author(AUTHOR_1);
        let author_2 = parse_series_page_author(AUTHOR_2);

        assert_eq!(author, "instantmiso".to_string());
        assert_eq!(author_1, "HYBE".to_string());
        assert_eq!(author_2, "PASA , TARU ...");
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

        let result = parse_series_page_title(TITLE);

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

        let result = parse_genre(GENRE);

        assert_eq!(result, "Romance");
    }
}
