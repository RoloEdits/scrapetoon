use core::time;
use scraper::{ElementRef, Html, Selector};
use std::thread;
use project_core::ResponseFactory;

use crate::parse_chapter_list::parse_chapter_list_pages;

use super::*;

pub async fn series_info(end: u16, input_url: &str) -> SeriesInfo {
    let (genre, _id) = parse_url_info(input_url);

    let mut chapter_info_list = LinkedList::new();

    let (title, author, status, release_day, views, subscribers, rating) = parse_series_page_info(input_url).await;

    parse_chapter_list_pages(end, input_url, &mut chapter_info_list).await;

    SeriesInfo {
        title,
        author,
        genre,
        status,
        release_day,
        chapter_info_list,
        views,
        subscribers,
        rating,
    }
}

// Series Helpers
fn parse_url_info(url: &str) -> (String, u16) {
    let reg = regex![
        r"https://www.webtoons.com/../(?P<genre>.+)/(?P<title>.+)/list\?title_no=(?P<id>\d+)"
    ];

    let cap = reg.captures(url).unwrap();

    (cap["genre"].to_string(), cap["id"].parse::<u16>().unwrap())
}

// Series Page
async fn parse_series_page_info(url: &str) -> (String, String, String, String, u64, u32, f32) {
    let html = if let Ok(html_response) = ResponseFactory::get(url).await {
        html_response
    } else {
        panic!("Error conncting to URL webpage: {}", url)
    }
    .text()
    .await
    .expect("Error getting HTML from response");

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
        result.push_str(word)
    }

    result.replace(':', ": ")
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
            (sub_text
                .replace('M', "")
                .parse::<f32>()
                .unwrap_or_else(|_| {
                    panic!("Error! Couldn't get subscriber count. Value ={}", sub_text)
                })
                * 1_000_000.0) as u32
        }
        sub_text => sub_text
            .replace(',', "")
            .parse::<u32>()
            .unwrap_or_else(|_| {
                panic!("Error! Couldn't get subscriber count. Value ={}", sub_text)
            }),
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
            (sub_text
                .replace('M', "")
                .parse::<f32>()
                .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}", sub_text))
                * 1_000_000.0) as u64
        }
        sub_text if sub_text.ends_with('B') => {
            (sub_text
                .replace('B', "")
                .parse::<f32>()
                .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}", sub_text))
                * 1_000_000_000.0) as u64
        }
        sub_text => sub_text
            .replace(',', "")
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}", sub_text)),
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

    let (day, status) = match result {
        sub_text if sub_text.starts_with("SUN") => ("sunday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("MON") => ("monday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("TUE") => ("tuesday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("WED") => ("wednesday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("THU") => ("thursday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("FRI") => ("friday".to_string(), "ongoing".to_string()),
        sub_text if sub_text.starts_with("SAT") => ("saturday".to_string(), "ongoing".to_string()),
        _ => ("completed".to_string(), "completed".to_string()),
    };

    (day, status)
}

fn parse_series_page_author(html: &str) -> String {
    let html = Html::parse_document(html);
    let author_with_link_selector = Selector::parse(r"a.author").unwrap();
    let author_without_link_selector = Selector::parse(r"div.author_area").unwrap();

    let mut author_element = html.select(&author_with_link_selector);
    let author_fragment: ElementRef = match author_element.next() {
        Some(some) => some,

        None => {
            let without_link = html.select(&author_without_link_selector).next().unwrap();

            without_link
        }
    };

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

        assert_eq!(result, 9.86);
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

        let result_1 = parse_series_page_views(VIEWS_1);
        let result_2 = parse_series_page_views(VIEWS_2);
        let result_3 = parse_series_page_views(VIEWS_3);

        assert_eq!(result_1, 1_000_000_000);
        assert_eq!(result_2, 245_678);
        assert_eq!(result_3, 1_100_000);
    }

    #[test]
    fn should_parse_release_day_and_is_completed() {
        const DAY: &str =
            r##"<p class="day_info"><span class="txt_ico_up">UP</span>EVERY MONDAY</p>"##;

        const COMPLETED: &str =
            r##"<p class="day_info"><span class="txt_ico_up">UP</span>COMPLETED</p>"##;

        let monday = parse_series_page_release_day_and_status(DAY);
        let completed = parse_series_page_release_day_and_status(COMPLETED);

        assert_eq!(monday, ("monday".to_string(), "ongoing".to_string()));
        assert_eq!(
            completed,
            ("completed".to_string(), "completed".to_string())
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
        assert_eq!(author_2, "PASA , TARU ...")
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
}
