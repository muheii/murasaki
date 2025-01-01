use crate::database::Database;
use crate::types::{
    ApiError, ContentSearchResult, ContentType, JikanResponse, Result, StorageItem, VndbResponse,
};
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
