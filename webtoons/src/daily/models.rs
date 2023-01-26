use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Daily {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub total_likes: u32,
    pub status: String,
    pub day: String,
}
