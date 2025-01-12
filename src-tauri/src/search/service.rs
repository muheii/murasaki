use super::types::{JikanResponse, VndbResponse};
use crate::common::types::{Content, ContentType};
use anyhow::Result;

pub async fn search_anime(query: &str) -> Result<Vec<Content>> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
    let resp = reqwest::get(&url).await?.json::<JikanResponse>().await?;

    Ok(resp
        .data
        .into_iter()
        .map(|anime| Content::from((anime, ContentType::Anime)))
        .collect())
}

pub async fn search_vn(query: &str) -> Result<Vec<Content>> {
    let filters = vec!["search", "=", query];
    let fields = "title, image.url, description, alttitle, released, length_minutes, length_votes, rating, votecount";

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
        .map(|vn| Content::from((vn, ContentType::Vn)))
        .collect())
}
