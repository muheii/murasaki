use anyhow::Result;
use regex::Regex;
use std::fs;

use super::types::Episode;

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
