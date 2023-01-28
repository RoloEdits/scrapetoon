use super::comments::models::UserComment;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Chapter {
    pub number: u16,
    pub likes: u32,
    pub length: Option<u32>,
    pub comments: u32,
    pub replies: u32,
    pub season: Option<u8>,
    pub season_chapter: Option<u16>,
    pub arc: Option<String>,
    pub user_comments: Vec<UserComment>,
    pub published: Option<String>,
    pub scraped: String,
}
