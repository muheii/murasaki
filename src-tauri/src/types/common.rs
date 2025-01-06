use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ContentType {
    Anime,
    Vn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub id: u64,
    pub external_id: String,
    pub content_type: ContentType,
    pub title: String,
    pub title_japanese: Option<String>,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub image_path: String,
    pub release_date: Option<String>,
    pub episodes: Option<u16>,
    pub length_minutes: Option<u64>,
    pub length_votes: Option<u64>,
    pub rating: Option<f32>,
    pub votecount: Option<u64>,
}
