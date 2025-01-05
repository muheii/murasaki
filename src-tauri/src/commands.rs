use crate::config::Config;
use crate::database::Database;
use crate::search::scan_anime_episodes;
use crate::types::common::{ContentSearchResult, ContentType};
use crate::types::database::StorageItem;
use anyhow_tauri::TAResult;
use tauri::State;

#[tauri::command]
pub async fn search_content(
    content_type: ContentType,
    query: &str,
) -> TAResult<Vec<ContentSearchResult>> {
    Ok(content_type.search(query).await?)
}

#[tauri::command]
pub async fn add_to_library(
    db: State<'_, Database>,
    search_result: ContentSearchResult,
) -> TAResult<()> {
    let storage_item = StorageItem::from(search_result);

    if storage_item.content_type == ContentType::Anime {
        let result = scan_anime_episodes(storage_item.id, &storage_item.content_path);
        println!("{:?}", result);
    }

    Ok(db.write_item(&storage_item)?)
}

#[tauri::command]
pub async fn get_from_library(
    db: State<'_, Database>,
    content_type: ContentType,
) -> TAResult<Vec<StorageItem>> {
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
pub async fn launch_content(db: State<'_, Database>, storage_item: StorageItem) -> TAResult<()> {
    let user_activity = storage_item.content_type.launch(&storage_item).await?;
    db.write_user_activity(&user_activity)?;
    Ok(())
}
