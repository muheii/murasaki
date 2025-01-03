use serde::{Deserialize, Serialize};

use super::common::{ContentSearchResult, ContentType};

// Database storage type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageItem {
    pub id: u64,
    pub content_type: ContentType,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail_path: String,
    pub content_path: String,
}

pub struct UserActivity {
    pub id: u64,
    pub date: String,
    pub minutes_watched: u64,
    pub minutes_read: u64,
    pub characters_read: u64,
    pub content_id: u64,
}

impl From<ContentSearchResult> for StorageItem {
    fn from(result: ContentSearchResult) -> Self {
        Self {
            id: 0,
            content_type: result.content_type,
            external_id: result.external_id,
            name: result.title,
            description: result.description,
            thumbnail_path: result.image_url,
            content_path: "placeholder".to_string(),
        }
    }
}
