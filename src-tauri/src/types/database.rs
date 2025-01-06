use serde::{Deserialize, Serialize};

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
