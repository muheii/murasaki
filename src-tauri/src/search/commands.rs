use anyhow_tauri::TAResult;

use crate::{common::types::Content, common::types::ContentType};

use super::service::{search_anime, search_vn};

#[tauri::command]
pub async fn search_content(content_type: ContentType, query: &str) -> TAResult<Vec<Content>> {
    let result = match content_type {
        ContentType::Anime => search_anime(query).await?,
        ContentType::Vn => search_vn(query).await?,
    };

    Ok(result)
}
