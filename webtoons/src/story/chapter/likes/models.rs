use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(alias = "contents")]
    pub chapters: Vec<Chapter>,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    #[serde(alias = "reactions")]
    pub reactions: Vec<Reactions>,
}

#[derive(Deserialize, Debug)]
pub struct Reactions {
    #[serde(alias = "count")]
    pub count: u32,
}
