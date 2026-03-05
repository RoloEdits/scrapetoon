use serde::Serialize;

#[derive(Serialize)]
pub struct Stats<'a> {
    pub id: u32,
    pub creator: &'a str,
    pub title: &'a str,
    pub genre: &'a str,
    pub views: u64,
    pub subscribers: u32,
    pub episode: u16,
    pub likes: u32,
    pub comments: u32,
    pub replies: u32,
}
