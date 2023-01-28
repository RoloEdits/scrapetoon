use super::chapter::models::Chapter;

#[derive(Debug, Clone)]
pub struct Story<T: Clone + Send> {
    pub story_page: StoryPage,
    pub chapters: Vec<Chapter<T>>,
}

#[derive(Debug, Clone)]
pub struct StoryPage {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub status: String,
    pub release_day: String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
}
