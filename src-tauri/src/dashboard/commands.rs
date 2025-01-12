use anyhow_tauri::TAResult;
use tauri::State;

use crate::{common::database::Database, dashboard::types::ActivityStats};

#[tauri::command]
pub async fn get_activity_stats(
    db: State<'_, Database>,
    start_date: &str,
    end_date: &str,
) -> TAResult<ActivityStats> {
    Ok(db.get_activity_stats(start_date, end_date)?)
}
