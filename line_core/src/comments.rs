use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};
use std::collections::LinkedList;

use crate::UserComment;

///# Panics
///
/// Will panic if there is no chapter number to parse, or if there is a non number attempting to be parsed..
#[must_use]
pub fn parse_chapter_number(html: &Html) -> u16 {
    let chapter_number_selector = Selector::parse("span.tx").unwrap();

    let chapter_number = html
        .select(&chapter_number_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    chapter_number.replace('#', "").parse::<u16>().unwrap()
}

///# Panics
///
/// Will panic if there is no comment number to parse, or if there is a non number attempting to be parsed..
#[must_use]
pub fn parse_comment_count(html: &Html) -> u32 {
    let comment_amount_selector = Selector::parse(r#"span[class="u_cbox_count"]"#).unwrap();

    let chapter_number = html
        .select(&comment_amount_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0];

    chapter_number.replace(',', "").parse::<u32>().unwrap()
}

///# Panics
///
/// Will panic if there is no user comments to parse.
#[must_use]
pub fn parse_users(html: &Html) -> LinkedList<UserComment> {
    let mut comments: LinkedList<UserComment> = LinkedList::new();

    let comment_list_selector = Selector::parse(r#"ul[class="u_cbox_list"]>li"#).unwrap();

    let user_selector = Selector::parse(r#"span[class="u_cbox_nick"]"#).unwrap();
    let comment_text_selector = Selector::parse(r#"span[class="u_cbox_contents"]"#).unwrap();
    let comment_upvote_selector = Selector::parse(r#"em[class="u_cbox_cnt_recomm"]"#).unwrap();
    let comment_downvote_selector = Selector::parse(r#"em[class="u_cbox_cnt_unrecomm"]"#).unwrap();
    let comment_reply_count_selector =
        Selector::parse(r#"span[class="u_cbox_reply_cnt"]"#).unwrap();
    let comment_date_selector = Selector::parse(r#"span[class="u_cbox_date"]"#).unwrap();

    for user_comment in html.select(&comment_list_selector) {
        let user = parse_user(user_comment, &user_selector);
        let comment_body = parse_comment_body(user_comment, &comment_text_selector);
        let post_date = parse_comment_post_date(user_comment, &comment_date_selector);
        let upvotes = parse_comment_upvote(user_comment, &comment_upvote_selector);
        let downvotes = parse_comment_downvote(user_comment, &comment_downvote_selector);
        let reply_count = parse_comment_reply_count(user_comment, &comment_reply_count_selector);

        let user_comment = UserComment::new(
            user,
            comment_body,
            post_date,
            upvotes,
            downvotes,
            reply_count,
        );

        comments.push_back(user_comment);
    }

    comments
}

fn parse_user(user_comment: ElementRef, user_selector: &Selector) -> String {
    let comment_text = match user_comment.select(user_selector).next() {
        Some(text) => text,
        None => return "This user has been deleted.".to_string(),
    }
    .text()
    .collect::<Vec<_>>()[0]
        .to_string();

    comment_text
}

fn parse_comment_post_date(user_comment: ElementRef, comment_date_selector: &Selector) -> String {
    let comment_date = match user_comment.select(comment_date_selector).next() {
        Some(date) => date,
        None => return String::new(),
    }
    .text()
    .collect::<Vec<_>>()[0];
    let datetime = NaiveDate::parse_from_str(comment_date, "%b %e, %Y").unwrap();

    // %b %e, %Y -> Jun 3, 2022
    // %b %d, %Y -> Jun 03, 2022
    // %F -> 2022-06-03 (ISO 8601)
    let formatted_date = datetime.format("%F").to_string();
    formatted_date
}

///# Panics
///
/// Will panic if there is no user reply count to parse, or if there is a non number attempting to be parsed.
#[must_use]
pub fn parse_comment_reply_count(
    user_comment: ElementRef,
    comment_reply_count_selector: &Selector,
) -> u16 {
    let comment_reply_count = match user_comment.select(comment_reply_count_selector).next() {
        Some(element) => element,
        // When there are no replies.
        None => return 0,
    }
    .text()
    .collect::<Vec<_>>()[0]
        // Once replies get past 999, a '+' is added. Need to remove to parse.
        .replace('+', "")
        .parse::<u16>()
        .unwrap();
    comment_reply_count
}

///# Panics
///
/// Will panic if there is no downvote to parse, or if there is a non number attempting to be parsed.
#[must_use]
pub fn parse_comment_downvote(
    user_comment: ElementRef,
    comment_downvote_selector: &Selector,
) -> u32 {
    let comment_downvote = match user_comment.select(comment_downvote_selector).next() {
        Some(upvote) => upvote,
        None => return 0,
    }
    .text()
    .collect::<Vec<_>>()[0]
        .parse::<u32>()
        .unwrap();
    comment_downvote
}

///# Panics
///
/// Will panic if there is no upvote to parse, or if there is a non number attempting to be parsed.
#[must_use]
pub fn parse_comment_upvote(user_comment: ElementRef, comment_upvote_selector: &Selector) -> u32 {
    let comment_upvote = match user_comment.select(comment_upvote_selector).next() {
        Some(upvote) => upvote,
        None => return 0,
    }
    .text()
    .collect::<Vec<_>>()[0]
        .parse::<u32>()
        .unwrap();
    comment_upvote
}

#[must_use]
pub fn parse_comment_body(user_comment: ElementRef, comment_text_selector: &Selector) -> String {
    let comment_text = match user_comment.select(comment_text_selector).next() {
        Some(text) => text,
        None => return "This comment has been deleted.".to_string(),
    }
    .text()
    .collect::<Vec<_>>()[0]
        .to_string();
    comment_text.replace('\n', " ")
}

#[cfg(test)]
mod parse_comments_tests {
    use scraper::Html;

    use super::*;

    #[test]
    fn should_parse_chapter_number() {
        const CHAPTER_NUMBER: &str =
            r##"<span class="tx _btnOpenEpisodeList NPI=a:current,g:en_en">#550</span>"##;

        let html = Html::parse_document(CHAPTER_NUMBER);

        let result = parse_chapter_number(&html);

        assert_eq!(result, 550);
    }

    #[test]
    fn should_parse_comment_replies() {
        const COMMENT: &str = r##"<div class="u_cbox_area">
        <div class="u_cbox_info">
        <span class="u_cbox_info_main">
        <span class="u_cbox_sns_icons u_cbox_sns_facebook">Facebook</span>
        <span class="u_cbox_name"><span class="u_cbox_name_area">
        <span class="u_cbox_nick_area">
        <span class="u_cbox_nick">Kayla Hoang</span>
        </span>
        </span>
        </span>
        </span>
        <span class="u_cbox_info_sub">
        </span>
        </div><div class="u_cbox_text_wrap">
        <span class="u_cbox_contents" data-lang="en">Tower of God is my life, and my favorite webtoon! it's just...UGH! I can't explain. YOU HAVE TO READ IT😵😂😱</span>
        </div>
        <div class="u_cbox_info_base"><span class="u_cbox_date" data-value="2015-09-22T08:43:23+0900">Sep 21, 2015</span>
        <span class="u_cbox_work_main">
        <a href="#" class="u_cbox_btn_report" data-action="report#request" data-param="commentNo:'1427616',objectId:'w_95_2'" data-log="RPC.report"><span class="u_cbox_ico_bar">
        </span><span class="u_cbox_ico_report"></span><span class="u_cbox_in_report">Report</span>
        </a></span></div><div class="u_cbox_tool">
        <a href="#" role="button" aria-expanded="false" class="u_cbox_btn_reply" data-action="reply#toggle" data-param="1427616" data-log="RPC.replyopen#RPC.replyclose">
        <strong class="u_cbox_reply_txt">Reply</strong><span class="u_cbox_reply_cnt u_vc">0</span></a><div class="u_cbox_recomm_set"><strong class="u_vc">Like/Dislike</strong><a href="#" data-action="vote" data-param="mine:false,commentNo:'1427616',voteStatus:'SYMPATHY',objectId:'w_95_2',ticket:'webtoon'" data-log="RPC.sym#RPC.unsym" class="u_cbox_btn_recomm">
        <span class="u_cbox_ico_recomm">Like</span><em class="u_cbox_cnt_recomm">49</em></a><a href="#" data-action="vote" data-param="mine:false,commentNo:'1427616',voteStatus:'ANTIPATHY',objectId:'w_95_2',ticket:'webtoon'" data-log="RPC.dis#RPC.undis" class="u_cbox_btn_unrecomm"><span class="u_cbox_ico_unrecomm">Dislike</span><em class="u_cbox_cnt_unrecomm">0</em>
        </a></div></div><span class="u_cbox_comment_frame"><span class="u_cbox_ico_tip"></span><span class="u_cbox_comment_frame_top"><span class="u_cbox_comment_bg_r"></span><span class="u_cbox_comment_bg_l"></span></span><span class="u_cbox_comment_frame_bottom"><span class="u_cbox_comment_bg_r"></span><span class="u_cbox_comment_bg_l">
        </span></span></span></div>"##;

        let html = Html::parse_document(COMMENT);

        let comment_reply_count_selector =
            Selector::parse(r#"span[class="u_cbox_reply_cnt"]"#).unwrap();

        let comment_list_selector = Selector::parse(r#"div.u_cbox_area"#).unwrap();

        for user_comment in html.select(&comment_list_selector) {
            let result = parse_comment_reply_count(user_comment, &comment_reply_count_selector);
            assert_eq!(result, 0);
        }
    }

    #[test]
    fn should_parse_chapter_comments_count() {
        const NUMBER: &str = r##"<div class="u_cbox_head">
        <h5 class="u_cbox_title">Comments</h5><span class="u_cbox_count">7,391</span><button type="button"
            class="u_cbox_btn_refresh" data-action="count#refresh" data-log="RPO.refresh"><span
                class="u_cbox_ico_refresh"></span><span class="u_cbox_txt_refresh">Refresh</span></button>
        <div class="u_cbox_head_tools"></div>
    </div>
    <div class="u_cbox_write_wrap">
        <div class="u_cbox_write_box u_cbox_type_logged_out">
            <form>
                <fieldset>
                    <legend class="u_vc">Enter comments</legend>
                    <div class="u_cbox_write">
                        <div class="u_cbox_write_inner">
                            <div class="u_cbox_write_area"><strong class="u_vc">Enter comments</strong>
                                <div class="u_cbox_inbox"><textarea title="Comments" id="cbox_module__write_textarea"
                                        class="u_cbox_text" rows="3" cols="30" data-log="RPC.input"></textarea><label
                                        for="cbox_module__write_textarea" class="u_cbox_guide"
                                        data-action="write#placeholder" data-param="@event">Please <a href="#"
                                            class="u_cbox_link">log in</a> to leave a comment.</label></div>
                            </div>
                            <div class="u_cbox_upload_image" style="display:none"><span
                                    class="u_cbox_upload_image_wrap fileButton browsebutton _cboxImageSelect"><span
                                        class="u-cbox-browse-box"><input class="u-cbox-browse-file-input" type="file"
                                            name="browse" accept="image/*" title="Add image"></span><a href="#"
                                        class="u_cbox_upload_thumb_link u-cbox-browse-button" data-log="RPP.add"><span
                                            class="u_cbox_upload_thumb_add">Add image</span><span
                                            class="u_cbox_upload_thumb_mask"></span></a></span></div>
                            <div class="u_cbox_upload_sticker" style="display:none"></div>
                            <div class="u_cbox_write_count"><span class="u_vc">Number of letters that can be inserted
                                    currently</span><strong class="u_cbox_count_num">0</strong>/<span class="u_vc">Total
                                    number of letters that can be inserted</span><span class="u_cbox_write_total">500</span>
                            </div>
                            <div class="u_cbox_upload">
                                <div class="u_cbox_addition"></div><button type="button" class="u_cbox_btn_upload"
                                    data-action="write#request" data-log="RPC.write#RPC.reply"><span
                                        class="u_cbox_ico_upload"></span><span
                                        class="u_cbox_txt_upload">Post</span></button>
                            </div>
                        </div>
                    </div>
                </fieldset>
            </form>
        </div>
    </div>"##;

        let html = Html::parse_document(NUMBER);

        let result = parse_comment_count(&html);

        assert_eq!(result, 7_391);
    }

    #[test]
    fn should_parse_all_user_comments() {
        const USER_COMMENT_LIST: &str = r##"<ul class="u_cbox_list">
        <li class="u_cbox_comment cbox_module__comment_141421 _user_id_no_79cd2a40-6616-11e4-8314-000000000425"
            data-info="commentNo:'141421',deleted:false,best:true,visible:true,secret:false,manager:false,mine:false,report:undefined,blindReport:false,objectId:'w_95_1',replyLevel:1,parentCommentNo:'141421',pick:false">
            <div class="u_cbox_comment_box">
                <div class="u_cbox_area">
                    <div class="u_cbox_info"><span class="u_cbox_info_main"><span
                                class="u_cbox_sns_icons u_cbox_sns_facebook">Facebook</span><span class="u_cbox_name"><span
                                    class="u_cbox_name_area"><span class="u_cbox_nick_area"><span
                                            class="u_cbox_nick">주수한</span></span></span></span></span><span
                            class="u_cbox_info_sub"></span></div>
                    <div class="u_cbox_text_wrap"><span class="u_cbox_ico_best" style="">TOP</span><span
                            class="u_cbox_contents" data-lang="en">Hey Guys, this is the beginning of a legend.</span></div>
                    <div class="u_cbox_info_base"><span class="u_cbox_date" data-value="2014-11-07T09:42:04+0900">Nov 6,
                            2014</span><span class="u_cbox_work_main"><a href="#" class="u_cbox_btn_report"
                                data-action="report#request" data-param="commentNo:'141421',objectId:'w_95_1'"
                                data-log="RPC.report"><span class="u_cbox_ico_bar"></span><span
                                    class="u_cbox_ico_report"></span><span class="u_cbox_in_report">Report</span></a></span>
                    </div>
                    <div class="u_cbox_tool"><a href="#" role="button" aria-expanded="false" class="u_cbox_btn_reply"
                            data-action="reply#toggle" data-param="141421" data-log="RPC.replyopen#RPC.replyclose"><strong
                                class="u_cbox_reply_txt">Replies</strong><span class="u_cbox_reply_cnt">114</span></a>
                        <div class="u_cbox_recomm_set"><strong class="u_vc">Like/Dislike</strong><a href="#"
                                data-action="vote"
                                data-param="mine:false,commentNo:'141421',voteStatus:'SYMPATHY',objectId:'w_95_1',ticket:'webtoon'"
                                data-log="RPC.sym#RPC.unsym" class="u_cbox_btn_recomm"><span
                                    class="u_cbox_ico_recomm">Like</span><em class="u_cbox_cnt_recomm">63591</em></a><a
                                href="#" data-action="vote"
                                data-param="mine:false,commentNo:'141421',voteStatus:'ANTIPATHY',objectId:'w_95_1',ticket:'webtoon'"
                                data-log="RPC.dis#RPC.undis" class="u_cbox_btn_unrecomm"><span
                                    class="u_cbox_ico_unrecomm">Dislike</span><em class="u_cbox_cnt_unrecomm">295</em></a>
                        </div>
                    </div><span class="u_cbox_comment_frame"><span class="u_cbox_ico_tip"></span><span
                            class="u_cbox_comment_frame_top"><span class="u_cbox_comment_bg_r"></span><span
                                class="u_cbox_comment_bg_l"></span></span><span class="u_cbox_comment_frame_bottom"><span
                                class="u_cbox_comment_bg_r"></span><span class="u_cbox_comment_bg_l"></span></span></span>
                </div>
            </div>
            <div class="u_cbox_reply_area" style="display:none;"></div>
        </li>
        <li class="u_cbox_comment cbox_module__comment_1961605 _user_id_no_faffc290-7668-11e5-8764-00000000041d"
            data-info="commentNo:'1961605',deleted:false,best:true,visible:true,secret:false,manager:false,mine:false,report:undefined,blindReport:false,objectId:'w_95_1',replyLevel:1,parentCommentNo:'1961605',pick:false">
            <div class="u_cbox_comment_box">
                <div class="u_cbox_area">
                    <div class="u_cbox_info"><span class="u_cbox_info_main"><span
                                class="u_cbox_sns_icons u_cbox_sns_facebook">Facebook</span><span class="u_cbox_name"><span
                                    class="u_cbox_name_area"><span class="u_cbox_nick_area"><span class="u_cbox_nick">Joey
                                            Cole</span></span></span></span></span><span class="u_cbox_info_sub"></span>
                    </div>
                    <div class="u_cbox_text_wrap"><span class="u_cbox_ico_best" style="">TOP</span><span
                            class="u_cbox_contents" data-lang="en">I got a Webtoons account just so I could give this comic
                            a 10/10.</span></div>
                    <div class="u_cbox_info_base"><span class="u_cbox_date" data-value="2015-10-23T12:30:27+0900">Oct 22,
                            2015</span><span class="u_cbox_work_main"><a href="#" class="u_cbox_btn_report"
                                data-action="report#request" data-param="commentNo:'1961605',objectId:'w_95_1'"
                                data-log="RPC.report"><span class="u_cbox_ico_bar"></span><span
                                    class="u_cbox_ico_report"></span><span class="u_cbox_in_report">Report</span></a></span>
                    </div>
                    <div class="u_cbox_tool"><a href="#" role="button" aria-expanded="false" class="u_cbox_btn_reply"
                            data-action="reply#toggle" data-param="1961605" data-log="RPC.replyopen#RPC.replyclose"><strong
                                class="u_cbox_reply_txt">Replies</strong><span class="u_cbox_reply_cnt">34</span></a>
                        <div class="u_cbox_recomm_set"><strong class="u_vc">Like/Dislike</strong><a href="#"
                                data-action="vote"
                                data-param="mine:false,commentNo:'1961605',voteStatus:'SYMPATHY',objectId:'w_95_1',ticket:'webtoon'"
                                data-log="RPC.sym#RPC.unsym" class="u_cbox_btn_recomm"><span
                                    class="u_cbox_ico_recomm">Like</span><em class="u_cbox_cnt_recomm">43103</em></a><a
                                href="#" data-action="vote"
                                data-param="mine:false,commentNo:'1961605',voteStatus:'ANTIPATHY',objectId:'w_95_1',ticket:'webtoon'"
                                data-log="RPC.dis#RPC.undis" class="u_cbox_btn_unrecomm"><span
                                    class="u_cbox_ico_unrecomm">Dislike</span><em class="u_cbox_cnt_unrecomm">402</em></a>
                        </div>
                    </div><span class="u_cbox_comment_frame"><span class="u_cbox_ico_tip"></span><span
                            class="u_cbox_comment_frame_top"><span class="u_cbox_comment_bg_r"></span><span
                                class="u_cbox_comment_bg_l"></span></span><span class="u_cbox_comment_frame_bottom"><span
                                class="u_cbox_comment_bg_r"></span><span class="u_cbox_comment_bg_l"></span></span></span>
                </div>
            </div>
            <div class="u_cbox_reply_area" style="display:none;"></div>
        </li>
    </ul>"##;

        let html = Html::parse_document(USER_COMMENT_LIST);

        let result = parse_users(&html);

        assert_eq!(result.len(), 2);

        let check = result.into_iter().next().unwrap();

        assert_eq!(check.body, "Hey Guys, this is the beginning of a legend.");
        assert_eq!(check.upvotes, 63_591);
        assert_eq!(check.downvotes, 295);
        assert_eq!(check.reply_count, 114);
        assert_eq!(check.post_date, "2014-11-06");
    }
}
