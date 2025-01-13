use serde::{Deserialize, Serialize};

use crate::common::types::{Content, ContentType};

#[derive(Deserialize, Debug)]
pub struct JikanResponse {
    pub data: Vec<Anime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    pub mal_id: u64,
    pub url: String,
    pub images: JikanImages,
    pub title: String,
    pub title_japanese: Option<String>,
    pub episodes: Option<u16>,
    pub score: Option<f32>,
    pub scored_by: Option<u64>,
    pub synopsis: Option<String>,
    pub season: Option<String>,
    pub year: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JikanImages {
    pub jpg: JikanJpg,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JikanJpg {
    pub large_image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct VndbResponse {
    pub results: Vec<Vn>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vn {
    pub title: String,
    pub alttitle: Option<String>,
    pub image: VnImage,
    pub id: String,
    pub description: Option<String>,
    pub length_minutes: Option<u64>,
    pub length_votes: Option<u64>,
    pub rating: Option<f32>,
    pub votecount: Option<u64>,
    pub released: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VnImage {
    pub url: String,
}

impl From<(Anime, ContentType)> for Content {
    fn from((anime, content_type): (Anime, ContentType)) -> Self {
        let release_date = match (anime.season, anime.year) {
            (Some(season), Some(year)) => {
                let season = match season.as_str() {
                    "winter" => "Winter",
                    "spring" => "Spring",
                    "summer" => "Summer",
                    "fall" => "Fall",
                    s => s,
                };
                Some(format!("{} {}", season, year))
            }
            _ => None,
        };

        Self {
            id: 0,
            content_type,
            external_id: anime.mal_id.to_string(),
            image_path: anime.images.jpg.large_image_url,
            title: anime.title,
            title_japanese: anime.title_japanese,
            description: anime.synopsis,
            episodes: anime.episodes,
            release_date,
            rating: anime.score,
            votecount: anime.scored_by,
            length_minutes: None,
            length_votes: None,
            file_path: None,
        }
    }
}

impl From<(Vn, ContentType)> for Content {
    fn from((vn, content_type): (Vn, ContentType)) -> Self {
        Self {
            id: 0,
            content_type,
            external_id: vn.id,
            image_path: vn.image.url,
            title: vn.title,
            title_japanese: vn.alttitle,
            description: vn.description,
            episodes: None,
            release_date: vn.released,
            rating: vn.rating.map(|r| r / 10.0), // Map to handle the Option
            votecount: vn.votecount,
            length_minutes: vn.length_minutes,
            length_votes: vn.length_votes,
            file_path: None,
        }
    }
}
