use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(alias = "result")]
    pub result: Result,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Result {
    #[serde(alias = "count")]
    pub count: Count,
    #[serde(alias = "commentList")]
    pub comments: Vec<UserComment>,
    #[serde(alias = "bestList")]
    pub top_comments: [UserComment; 3],
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Count {
    #[serde(alias = "reply")]
    pub replies: u32,
    #[serde(alias = "comment")]
    pub comments: u32,
    #[serde(alias = "total")]
    pub total: u32,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct UserComment {
    #[serde(alias = "userName")]
    pub username: String,
    #[serde(alias = "replyCount")]
    pub replies: u32,
    #[serde(alias = "sympathyCount")]
    pub upvotes: u32,
    #[serde(alias = "antipathyCount")]
    pub downvotes: u32,
    #[serde(alias = "contents")]
    pub contents: String,
    #[serde(alias = "profileType")]
    pub profile_type: String,
    #[serde(alias = "idProvider")]
    pub auth_provider: String,
    #[serde(alias = "country")]
    pub country: String,
    #[serde(alias = "regTime")]
    pub post_date: String,
}

// #[derive(Serialize, Debug, Deserialize)]
// pub struct UserComment {
//     pub user: Option<String>,
//     pub contents: Option<String>,
//     pub post_date: Option<String>,
//     pub upvotes: Option<u32>,
//     pub downvotes: Option<u32>,
//     pub reply_count: Option<u16>,
// }
//
// impl UserComment {
//     #[must_use]
//     pub const fn new(
//         user: Option<String>,
//         body: Option<String>,
//         post_date: Option<String>,
//         upvotes: Option<u32>,
//         downvotes: Option<u32>,
//         reply_count: Option<u16>,
//     ) -> Self {
//         Self {
//             user,
//             contents: body,
//             post_date,
//             upvotes,
//             downvotes,
//             reply_count,
//         }
//     }
// }
