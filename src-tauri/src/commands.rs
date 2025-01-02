use crate::config::Config;
use crate::database::Database;
use crate::types::{
    ApiError, ContentSearchResult, ContentType, JikanResponse, Result, StorageItem, TimeEntry,
    VndbResponse,
};
use chrono::Local;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::time::Instant;
use tauri::State;

#[tauri::command]
pub async fn search_content(
    content_type: ContentType,
    query: &str,
) -> Result<Vec<ContentSearchResult>> {
    match content_type {
        ContentType::Anime => {
            let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
            let resp = reqwest::get(&url)
                .await
                .map_err(|e| ApiError::FetchError(e.to_string()))?
                .json::<JikanResponse>()
                .await
                .map_err(|e| ApiError::ParseError(e.to_string()))?;

            Ok(resp
                .data
                .into_iter()
                .map(|anime| ContentSearchResult::from((anime, ContentType::Anime)))
                .collect())
        }
        ContentType::Vn => {
            let filters = vec!["search", "=", query];
            let fields = "title, image.url, description";

            let payload = serde_json::json!({
                "filters": filters,
                "fields": fields,
            });

            let client = reqwest::Client::new();
            let resp = client
                .post("https://api.vndb.org/kana/vn")
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| ApiError::FetchError(e.to_string()))?
                .json::<VndbResponse>()
                .await
                .map_err(|e| ApiError::ParseError(e.to_string()))?;

            Ok(resp
                .results
                .into_iter()
                .map(|vn| ContentSearchResult::from((vn, ContentType::Vn)))
                .collect())
        }
    }
}

#[tauri::command]
pub async fn add_to_library(
    db: State<'_, Database>,
    search_result: ContentSearchResult,
) -> Result<()> {
    let storage_item = StorageItem::from(search_result);
    db.write_item(&storage_item)
}

#[tauri::command]
pub async fn get_from_library(
    db: State<'_, Database>,
    content_type: ContentType,
) -> Result<Vec<StorageItem>> {
    db.read_items(content_type)
}

#[tauri::command]
pub async fn load_config() -> Result<Config> {
    Config::load()
}

#[tauri::command]
pub async fn save_config(config: Config) -> Result<()> {
    config.save()
}

#[tauri::command]
pub async fn launch_vn(db: State<'_, Database>, storage_item: StorageItem) -> Result<()> {
    let start_time = Local::now();
    let start_instant = Instant::now();

    let mut entry = TimeEntry {
        id: 0,
        content_id: storage_item.id,
        start_time: start_time.to_rfc3339(),
        duration: 0,
    };

    let exe_path = std::path::Path::new(&storage_item.executable_path);
    let working_dir = exe_path
        .parent()
        .ok_or_else(|| ApiError::LaunchError("Invalid executable path".to_string()))?;

    let mut command = Command::new(exe_path);

    // Change the working directory to avoid errors from the VN
    command.current_dir(working_dir);

    #[cfg(target_os = "windows")]
    command.creation_flags(winapi::um::winbase::DETACHED_PROCESS);

    if let Ok(mut child) = command.spawn() {
        child
            .wait()
            .map_err(|e| ApiError::LaunchError(e.to_string()))?;
        entry.duration = start_instant.elapsed().as_secs();
        db.log_time_entry(&entry)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}
