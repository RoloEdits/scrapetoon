mod models;

use anyhow::{Context, Result};
use models::Response;
use reqwest::blocking::Client;
use std::collections::HashMap;

pub fn parse(id: u32, chapter: u16) -> Result<u32> {
    let url = api_url_builder(id, chapter);

    let template: HashMap<String, String> = HashMap::new();

    let client = Client::new();

    let response = client
        .get(url)
        .header("referer", "https://www.webtoons.com/")
        .json(&template)
        .send()
        .context("Failed to connect to Likes API")?;

    let text = response.text().context("Failed to get JSON body")?;

    let json: Response =
        serde_json::from_str(&text).context("Failed to deserialize likes api json")?;

    let likes = json
        .chapters
        .first()
        .expect("Failed to take first chapter")
        .reactions
        .first()
        .expect("Failed to take first reaction")
        .count;

    Ok(likes)
}

fn api_url_builder(id: u32, chapter: u16) -> String {
    format!("https://global.apis.naver.com/lineWebtoon/like_neoid/v1/search/contents?q=LINEWEBTOON[w_{id}_{chapter}]&pool=comic")
}

#[cfg(test)]
mod parse_likes_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_build_json_spi_url() {
        let url = api_url_builder(95, 1);

        let expected = r"https://global.apis.naver.com/lineWebtoon/like_neoid/v1/search/contents?q=LINEWEBTOON[w_95_1]&pool=comic";

        assert_eq!(url, expected);
    }

    #[test]
    #[ignore]
    fn should_get_like_count_from_json_response() {
        let likes = parse(1320, 1).unwrap();

        assert_eq!(3191, likes);
    }
}
