use serde::{Deserialize, Serialize};

use crate::common::types::Content;

#[derive(Debug, Serialize)]
pub struct ContentWithStats {
    pub content: Content,
    pub last_active: Option<String>,
    pub total_minutes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: u64,
    pub content_id: i64,
    pub episode_number: u64,
    pub path: String,
    pub watched: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserActivity {
    pub id: u64,
    pub date: String,
    pub minutes_watched: u64,
    pub minutes_read: u64,
    pub characters_read: u64,
    pub content_id: u64,
}
