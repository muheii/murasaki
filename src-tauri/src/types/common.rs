use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    Anime,
    Vn,
}

// Intermediate unified search result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentSearchResult {
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub content_type: ContentType,
}
