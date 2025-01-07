use crate::config::Config;
use crate::database::Database;
use crate::search::scan_anime_episodes;
use crate::types::common::{Content, ContentType};
use crate::types::database::Episode;
use anyhow_tauri::TAResult;
use tauri::State;

#[tauri::command]
pub async fn search_content(content_type: ContentType, query: &str) -> TAResult<Vec<Content>> {
    Ok(content_type.search(query).await?)
}

#[tauri::command]
pub async fn add_to_library(db: State<'_, Database>, search_result: Content) -> TAResult<()> {
    if search_result.content_type == ContentType::Anime {
        if let Some(path) = &search_result.file_path {
            match scan_anime_episodes(&search_result.external_id, path) {
                Ok(episodes) => {
                    db.write_episodes(&episodes)?;
                }
                Err(e) => println!("Failed to scan episodes: {}", e),
            }
        }
    }

    Ok(db.write_item(&search_result)?)
}

#[tauri::command]
pub async fn get_from_library(
    db: State<'_, Database>,
    content_type: ContentType,
) -> TAResult<Vec<Content>> {
    Ok(db.read_items(content_type)?)
}

#[tauri::command]
pub async fn load_config() -> TAResult<Config> {
    Ok(Config::load()?)
}

#[tauri::command]
pub async fn save_config(config: Config) -> TAResult<()> {
    Ok(config.save()?)
}

#[tauri::command]
pub async fn launch_content(
    db: State<'_, Database>,
    content: Content,
    episode: Option<Episode>,
) -> TAResult<()> {
    let user_activity = content.content_type.launch(&content, episode).await?;
    db.write_user_activity(&user_activity)?;
    Ok(())
}

#[tauri::command]
pub async fn get_episodes(db: State<'_, Database>, external_id: &str) -> TAResult<Vec<Episode>> {
    Ok(db.read_episodes(external_id)?)
}
