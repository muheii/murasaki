use super::service;
use super::types::Config;
use anyhow_tauri::TAResult;

#[tauri::command]
pub async fn load_config() -> TAResult<Config> {
    Ok(service::load_config()?)
}

#[tauri::command]
pub async fn save_config(config: Config) -> TAResult<()> {
    Ok(service::save_config(&config)?)
}
