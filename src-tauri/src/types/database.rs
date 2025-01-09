use serde::{Deserialize, Serialize};

use super::common::Content;

#[derive(Deserialize, Serialize)]
pub struct UserActivity {
    pub id: u64,
    pub date: String,
    pub minutes_watched: u64,
    pub minutes_read: u64,
    pub characters_read: u64,
    pub content_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: u64,
    pub content_id: String,
    pub episode_number: u64,
    pub path: String,
    pub watched: bool,
}

#[derive(Debug, Serialize)]
pub struct ContentWithStats {
    pub content: Content,
    pub last_active: Option<String>,
    pub total_minutes: u64,
}
