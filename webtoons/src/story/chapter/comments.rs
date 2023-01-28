pub mod models;

use crate::factories::BlockingJsonReferClient;
use crate::story::chapter::comments::models::UserComment;
use anyhow::{Context, Result};
use models::Response;

type Comments = u32;
type Replies = u32;
type UserComments = Vec<UserComment>;

/// # Errors
pub fn parse(id: u32, chapter: u16) -> Result<(Comments, Replies, UserComments)> {
    let url = api_url_builder(id, chapter, 100, 1);

    let response =
        BlockingJsonReferClient::get(&url).context("Failed to connect to Comments Section API")?;

    let text = response.text().context("Failed to get JSON body")?;

    let cleaned = &text[10..text.len() - 2];

    // Range selection removes `_callback(` from the start and `);` from the end in an allocation free way
    let json: Response = serde_json::from_str(cleaned)?;

    let comments = json.result.count.comments;
    let replies = json.result.count.replies;
    let mut user_comments = json.result.top_comments;
    user_comments.extend_from_slice(&json.result.comments);

    for page in 2..=json.result.page_model.total_pages {
        let url = api_url_builder(id, chapter, 100, page);
        let response = BlockingJsonReferClient::get(&url)
            .context("Failed to connect to Comments Section API")?;
        let text = response.text().context("Failed to get JSON body")?;

        let cleaned = &text[10..text.len() - 2];

        // Range selection removes `_callback(` from the start and `);` from the end in an allocation free way
        let json: Response = serde_json::from_str(cleaned)?;

        user_comments.extend_from_slice(&json.result.comments);
    }

    Ok((comments, replies, user_comments))
}

/// # Info
///
/// 100 comments at once seems to be the max
#[must_use]
pub fn api_url_builder(id: u32, chapter: u16, comments: u16, page: u16) -> String {
    const START: &str = r"https://global.apis.naver.com/commentBox/cbox/web_neo_list_jsonp.json?ticket=webtoon&templateId=or_en&pool=cbox&lang=en";
    const END: &str =
        r"&listType=OBJECT&pageType=default&initialize=true&useAltSort=true&sort=FAVORITE";
    format!("{START}&objectId=w_{id}_{chapter}&pageSize={comments}{END}&page={page}")
}

#[cfg(test)]
mod parse_comments_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn should_get_comment_count_from_json_response() {
        let (comments, replies, _) = parse(1218, 137).unwrap();

        assert_eq!(3191, comments);
        assert_eq!(2584, replies);
    }

    #[test]
    fn should_build_json_spi_url() {
        let url = api_url_builder(95, 1, 15, 1);

        let expected = r"https://global.apis.naver.com/commentBox/cbox/web_neo_list_jsonp.json?ticket=webtoon&templateId=or_en&pool=cbox&lang=en&objectId=w_95_1&pageSize=15&listType=OBJECT&pageType=default&initialize=true&useAltSort=true&sort=FAVORITE&page=1";
        assert_eq!(url, expected);
    }
}
