use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterList {
    pub chapter: u16,
    pub likes: u32,
    pub date: String,
}
