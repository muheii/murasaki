use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivityStats {
    pub anime_minutes: u64,
    pub vn_minutes: u64,
    pub total_minutes: u64,
    pub active_days: u64,
    pub daily_activity: Vec<DailyActivity>,
}

#[derive(Debug, Serialize)]
pub struct DailyActivity {
    pub date: String,
    pub minutes: u64,
}
