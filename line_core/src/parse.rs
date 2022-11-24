// #![allow(unused_variables, dead_code)]

use chrono::NaiveDate;
use core::time;
use hashlink::LinkedHashSet;
use scraper::{ElementRef, Html, Selector};
use std::thread;

use cli_core::ProgressBarFactory;
use project_core::ResponseFactory;

use super::*;

pub fn series_info(end: u16, input_url: &str) -> SeriesInfo {
    let (genre, _id) = parse_url_info(input_url);

    let mut chapter_info_list = LinkedList::new();

    let (title, author, status, release_day, views, subscribers, rating) =
        parse_series_page_info(input_url);

    parse_chapter_list_pages(end, input_url, &mut chapter_info_list);

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
fn parse_series_page_info(url: &str) -> (String, String, String, String, u64, u32, f32) {
    let html = if let Ok(html_response) = ResponseFactory::get(url) {
        html_response
    } else {
        panic!("Error conncting to URL webpage: {}", url)
    }
    .text()
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

    result.replace(':', ": ").to_lowercase()
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
            (sub_text.replace('M', "").parse::<f32>().unwrap_or_else(|_| panic!("Error! Couldn't get subscriber count. Value ={}",
                sub_text)) * 1_000_000.0) as u32
        }
        sub_text => sub_text.replace(',', "").parse::<u32>().unwrap_or_else(|_| panic!("Error! Couldn't get subscriber count. Value ={}",
            sub_text)),
    }
}

fn parse_series_page_views(html: &str) -> u64 {
    let html = Html::parse_document(html);
    let views_selector = Selector::parse(r"em.cnt").unwrap();

    let views_element = html.select(&views_selector);

    let mut result: String = String::new();

    let iteration = 0;
    for element in views_element {
        if iteration == 0 {
            result = element.text().collect::<Vec<_>>()[0].to_string();
            break;
        }
    }



    match result {
        sub_text if sub_text.ends_with('M') => {
            (sub_text.replace('M', "").parse::<f32>().unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}",
                sub_text)) * 1_000_000.0) as u64
        }
        sub_text if sub_text.ends_with('B') => {
            (sub_text.replace('B', "").parse::<f32>().unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}",
                sub_text)) * 1_000_000_000.0) as u64
        }
        sub_text => sub_text.replace(',', "").parse::<u64>().unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}",
            sub_text)),
    }
}

fn parse_series_page_release_day_and_status(html: &str) -> (String, String) {
    let html = Html::parse_document(&html.replace("EVERY ", "").replace("UP", ""));
    let day_selector = Selector::parse(r"p.day_info").unwrap();

    let day_element = html.select(&day_selector);

    let mut result: String = String::new();

    let iteration = 0;
    for element in day_element {
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

    author_text.trim().to_lowercase()
}

// Chapter List
fn parse_chapter_list_pages(end: u16, input_url: &str, chapter_info: &mut LinkedList<ChapterInfo>) {
    let bar = ProgressBarFactory::get_bar(end);

    for page in 1..=end {
        let url = format!("{}&page={}", input_url, page);

        let html_response = match ResponseFactory::get(&url) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("Error connecting to webpage, attempting to save progress and exit...");

                if chapter_info.is_empty() {
                    panic!("Nothing to save, exiting.");
                }

                break;
            }
        }
        .text()
        .unwrap();

        parse_each_chapters_chapter_info(&html_response, chapter_info);

        thread::sleep(time::Duration::from_secs(3));

        bar.inc(1);
    }
}

fn parse_each_chapters_chapter_info(html: &str, chapter_info: &mut LinkedList<ChapterInfo>) {
    let html = Html::parse_document(html);

    let chapter_selector = Selector::parse("ul#_listUl>li").unwrap();

    for chapter in html.select(&chapter_selector) {
        let chapter_number = parse_chapter_number(&chapter);
        let likes = parse_chapter_like_amount(&chapter);
        let date = parse_chapter_date(&chapter);
        chapter_info.push_back(ChapterInfo {
            chapter_number,
            likes,
            date,
        })
    }
}

fn parse_chapter_number(html: &ElementRef) -> u16 {
    let chapter_number_selector = Selector::parse("span.tx").unwrap();

    let mut result: u16 = 0;

    for element in html.select(&chapter_number_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[0];

        result = chapter_number.replace('#', "").parse::<u16>().unwrap();
    }

    result
}

fn parse_chapter_like_amount(html: &ElementRef) -> u32 {
    let like_selector = Selector::parse(r#"span[class="like_area _likeitArea"]"#).unwrap();

    let mut result: u32 = 0;

    for element in html.select(&like_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[1];

        result = chapter_number.replace(',', "").parse::<u32>().unwrap();
    }

    result
}

fn parse_chapter_date(html: &ElementRef) -> String {
    let date_selector = Selector::parse("span.date").unwrap();

    let mut holder: Vec<&str> = Vec::with_capacity(9);

    for element in html.select(&date_selector) {
        let chapter_number = element.text().collect::<Vec<_>>()[0];

        holder.push(chapter_number);
    }

    let mut result: String = String::new();

    for date in holder {
        let datetime = NaiveDate::parse_from_str(date, "%b %e, %Y").unwrap();

        // %b %e, %Y -> Jun 3, 2022
        // %b %d, %Y -> Jun 03, 2022
        // %F -> 2022-06-03 (ISO 8601)
        let formatted = datetime.format("%F").to_string();

        result = formatted;
    }

    result
}

// Daily Schedule
pub fn parse_daily_schedule() -> LinkedHashSet<DailyScheduleInfo> {
    const DAILY_SCHEDULE: &str = "https://www.webtoons.com/en/dailySchedule";

    let mut series: LinkedHashSet<DailyScheduleInfo> = LinkedHashSet::new();

    let html = if let Ok(html_response) = ResponseFactory::get(DAILY_SCHEDULE) {
        html_response
    } else {
        panic!("Error conncting to URL webpage: {}", DAILY_SCHEDULE)
    }
    .text()
    .expect("Error getting HTML from response");

    let html = Html::parse_document(&html);
    let daily_card = Selector::parse("ul.daily_card>li").unwrap();

    for card in html.select(&daily_card) {
        let title = parse_daily_schedule_title(&card);
        let author = parse_daily_schedule_author(&card);
        let genre = parse_daily_schedule_genre(&card);
        let total_likes = parse_daily_schedule_total_likes(&card);
        let status = parse_daily_schedule_is_completed(&card);
        let release_day = parse_daily_schedule_release_day(&card);

        series.insert(DailyScheduleInfo {
            title,
            author,
            genre,
            total_likes,
            status,
            release_day,
        });
    }
    series
}

fn parse_daily_schedule_release_day(_card: &ElementRef) -> String {
    "not impimented yet".to_string()
}

fn parse_daily_schedule_is_completed(card: &ElementRef) -> String {
    let completed_selector = Selector::parse("p.icon_area").unwrap();

    let mut result = String::new();

    for status_check in card.select(&completed_selector) {
        let holder = status_check.text().collect::<Vec<_>>();

        if holder.is_empty() {
            return "ongoing".to_string();
        }

        result = holder[0].to_string();
    }



    match result {
        hiatus if hiatus == "HIATUS" => "hiatus".to_string(),
        completed if completed == "COMPLETED" => "completed".to_string(),
        _ => "ongoing".to_string(),
    }
}

fn parse_daily_schedule_total_likes(card: &ElementRef) -> u32 {
    let likes_selector = Selector::parse("em.grade_num").unwrap();

    let mut result = String::new();

    for likes in card.select(&likes_selector) {
        result = likes.text().collect::<Vec<_>>()[0].to_string()
    }



    match result {
        sub_text if sub_text.ends_with('M') => {
            (sub_text.replace('M', "").parse::<f32>().unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}",
                sub_text)) * 1_000_000.0) as u32
        }
        sub_text => sub_text.replace(',', "").parse::<u32>().unwrap_or_else(|_| panic!("Error! Couldn't get view count. Value ={}",
            sub_text)),
    }
}

fn parse_daily_schedule_genre(card: &ElementRef) -> String {
    let genre_selector = Selector::parse("p.genre").unwrap();

    let mut result = String::new();

    for genre in card.select(&genre_selector) {
        result = genre.text().collect::<Vec<_>>()[0].to_string()
    }

    result.to_lowercase()
}

fn parse_daily_schedule_author(card: &ElementRef) -> String {
    let author_selector = Selector::parse("p.author").unwrap();

    let mut result = String::new();

    for author in card.select(&author_selector) {
        result = author.text().collect::<Vec<_>>()[0].to_string()
    }

    result.to_lowercase()
}

fn parse_daily_schedule_title(card: &ElementRef) -> String {
    let title_selector = Selector::parse("p.subj").unwrap();

    let mut result = String::new();

    for title in card.select(&title_selector) {
        result = title.text().collect::<Vec<_>>()[0].to_string()
    }

    result.to_lowercase()
}

#[cfg(test)]
mod line_parsing_tests {
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

        let author = parse_series_page_author(AUTHOR);

        let author_1 = parse_series_page_author(AUTHOR_1);

        assert_eq!(author, "instantmiso".to_string());
        assert_eq!(author_1, "hybe".to_string());
    }

    #[test]
    fn should_parse_chapter_number() {
        const NUMBER: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">
						
        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>
            
            
            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(NUMBER);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = 0;

        for chapter in html.select(&chapter_selector) {
            result = parse_chapter_number(&chapter);
        }

        assert_eq!(result, 24);
    }

    #[test]
    fn should_parse_chapter_likes() {
        const LIKES: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">
						
        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>
            
            
            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(LIKES);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = 0;

        for chapter in html.select(&chapter_selector) {
            result = parse_chapter_like_amount(&chapter);
        }

        assert_eq!(result, 7_779);
    }

    #[test]
    fn should_parse_chapter_date() {
        const DATE: &str = r#"<li class="_episodeItem" id="episode_24" data-episode-no="24">
						
        <a href="https://www.webtoons.com/en/supernatural/to-tame-a-fire/episode-24/viewer?title_no=3763&amp;episode_no=24" class="NPI=a:list,i=3763,r=24,g:en_en">
            <span class="thmb">
                <img src="https://webtoon-phinf.pstatic.net/20221031_121/1667151253417biSNa_PNG/thumb_16671512222071190_Layer_4.png?type=q90" width="77" height="73" alt="Episode 24">
            </span>
            <span class="subj"><span>Episode 24</span></span>
            <span class="manage_blank"></span>
            <span class="date">Nov 20, 2022</span>
            
            
            <span class="like_area _likeitArea"><em class="ico_like _btnLike _likeMark">like</em>7,779</span>
            <span class="tx">#24</span>
        </a>
    </li>"#;

        let html = Html::parse_document(DATE);

        let chapter_selector = Selector::parse("li").unwrap();

        let mut result = String::new();

        for chapter in html.select(&chapter_selector) {
            result = parse_chapter_date(&chapter);
        }

        assert_eq!(result, "2022-11-20");
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

        assert_eq!(result, "dark moon: the blood altar");
    }
}
