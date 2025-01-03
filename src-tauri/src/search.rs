use crate::types::{
    api::JikanResponse,
    api::VndbResponse,
    common::{ContentSearchResult, ContentType},
};

use anyhow::Result;

impl ContentType {
    pub async fn search(&self, query: &str) -> Result<Vec<ContentSearchResult>> {
        match self {
            ContentType::Anime => Ok(search_anime(query).await?),
            ContentType::Vn => Ok(search_vn(query).await?),
        }
    }
}

async fn search_anime(query: &str) -> Result<Vec<ContentSearchResult>> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
    let resp = reqwest::get(&url).await?.json::<JikanResponse>().await?;

    Ok(resp
        .data
        .into_iter()
        .map(|anime| ContentSearchResult::from((anime, ContentType::Anime)))
        .collect())
}

async fn search_vn(query: &str) -> Result<Vec<ContentSearchResult>> {
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
        .await?
        .json::<VndbResponse>()
        .await?;

    Ok(resp
        .results
        .into_iter()
        .map(|vn| ContentSearchResult::from((vn, ContentType::Vn)))
        .collect())
}
