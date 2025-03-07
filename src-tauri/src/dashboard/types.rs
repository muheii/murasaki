use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivityStats {
    pub total_minutes: u64,

    // Activity category immersion time
    pub reading_minutes: u64,
    pub listening_minutes: u64,

    // Per content type minutes
    pub anime_minutes: u64,
    pub vn_minutes: u64,

    // Streaks
    pub current_streak: u64,
    pub best_streak: u64,

    pub daily_activity: Vec<DailyActivity>,
}

#[derive(Debug, Serialize)]
pub struct DailyActivity {
    pub date: String,
    pub total_minutes: u64,

    // Activity category immersion time
    pub reading_minutes: u64,
    pub listening_minutes: u64,

    // Per content type minutes
    pub anime_minutes: u64,
    pub vn_minutes: u64,
}
