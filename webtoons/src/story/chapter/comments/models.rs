use serde::Deserialize;

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
    pub top_comments: Vec<UserComment>,
    #[serde(alias = "pageModel")]
    pub page_model: PageModel,
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

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct PageModel {
    #[serde(alias = "pageSize")]
    pub page_size: u32,
    #[serde(alias = "totalPages")]
    pub total_pages: u16,
}

#[derive(Deserialize, Debug, Clone)]
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
    #[serde(alias = "regTimeGmt")]
    pub post_date: String,
    #[serde(alias = "idNo")]
    pub id_no: Option<String>,
    #[serde(alias = "userIdNo")]
    pub user_id_no: Option<String>,
    #[serde(alias = "profileUserId")]
    pub profile_user_id: Option<String>,
}
