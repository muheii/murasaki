use crate::types::{
    api::{JikanResponse, VndbResponse},
    common::{Content, ContentType},
    database::Episode,
};

use anyhow::Result;
use regex::Regex;
use std::fs;

impl ContentType {
    pub async fn search(&self, query: &str) -> Result<Vec<Content>> {
        match self {
            ContentType::Anime => Ok(search_anime(query).await?),
            ContentType::Vn => Ok(search_vn(query).await?),
        }
    }
}

async fn search_anime(query: &str) -> Result<Vec<Content>> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
    let resp = reqwest::get(&url).await?.json::<JikanResponse>().await?;

    Ok(resp
        .data
        .into_iter()
        .map(|anime| Content::from((anime, ContentType::Anime)))
        .collect())
}

async fn search_vn(query: &str) -> Result<Vec<Content>> {
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

pub fn scan_anime_episodes(content_id: &str, path: &str) -> Result<Vec<Episode>> {
    let standard_pattern =
        Regex::new(r"(?i)(?:e|episode|\s-\s)?\s*?(\d{1,3})(?:v\d)?(?:\s*?|$|\[|\.)").unwrap();
    // For cases like Serial Experiments Lain - S01E01.mkv
    let season_ep_pattern = Regex::new(r"(?i)S\d+E(\d+)").unwrap();

    let mut episodes: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let filename = path.file_name()?.to_string_lossy();

            // Prevent extras from being parsed
            if filename.to_lowercase().contains("special")
                || filename.to_lowercase().contains("nced")
                || filename.to_lowercase().contains("ncop")
            {
                return None;
            }

            // Prevent non-video files from being parsed
            if !filename.to_lowercase().ends_with(".mkv")
                && !filename.to_lowercase().ends_with(".mp4")
            {
                return None;
            }

            if let Some(cap) = season_ep_pattern.captures(&filename) {
                let episode_num = cap[1].parse::<u64>().ok()?;
                return Some((episode_num, path.to_string_lossy().to_string()));
            }

            standard_pattern.captures(&filename).and_then(|cap| {
                cap[1]
                    .parse::<u64>()
                    .ok()
                    .map(|episode_num| (episode_num, path.to_string_lossy().to_string()))
            })
        })
        .collect();

    episodes.sort_by_key(|(num, _)| *num);

    Ok(episodes
        .into_iter()
        .map(|(num, path)| Episode {
            id: 0,
            content_id: content_id.to_string(),
            episode_number: num,
            path,
            watched: false,
        })
        .collect())
}
