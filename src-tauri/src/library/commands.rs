use anyhow_tauri::TAResult;

use crate::{
    common::{
        database::Database,
        types::{Content, ContentType},
    },
    library::types::{ContentWithStats, Episode},
};

use super::service::scan_anime_episodes;
use tauri::State;

#[tauri::command]
pub async fn add_to_library(db: State<'_, Database>, content: Content) -> TAResult<()> {
    db.write_item(&content)?;

    if content.content_type == ContentType::Anime {
        if let Some(path) = &content.file_path {
            match scan_anime_episodes(&content.external_id, path) {
                Ok(episodes) => {
                    db.write_episodes(&episodes)?;
                }
                Err(e) => println!("Failed to scan episodes: {}", e),
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_library(
    db: State<'_, Database>,
    content_type: ContentType,
) -> TAResult<Vec<ContentWithStats>> {
    Ok(db.read_content_with_stats(content_type)?)
}

#[tauri::command]
pub async fn get_episodes(db: State<'_, Database>, external_id: &str) -> TAResult<Vec<Episode>> {
    Ok(db.read_episodes(external_id)?)
}

#[tauri::command]
pub async fn delete_content(db: State<'_, Database>, content_id: u64) -> TAResult<()> {
    Ok(db.delete_item(content_id)?)
}
