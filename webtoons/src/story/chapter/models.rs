use super::comments::models::UserComment;

#[derive(Debug, Clone)]
pub struct Chapter<T: Clone + Send> {
    pub number: u16,
    pub likes: u32,
    pub length: Option<u32>,
    pub comments: u32,
    pub replies: u32,
    pub season: Option<u8>,
    pub season_chapter: Option<u16>,
    pub arc: Option<String>,
    pub user_comments: Option<Vec<UserComment>>,
    pub published: Option<String>,
    pub timestamp: String,
    pub custom: Option<T>,
}
