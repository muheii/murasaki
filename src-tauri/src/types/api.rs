use serde::{Deserialize, Serialize};

use super::common::{ContentSearchResult, ContentType};

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
    pub episodes: u64,
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
    pub image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct VndbResponse {
    pub results: Vec<Vn>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vn {
    pub title: String,
    pub image: VnImage,
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VnImage {
    pub url: String,
}
impl From<(Anime, ContentType)> for ContentSearchResult {
    fn from((anime, content_type): (Anime, ContentType)) -> Self {
        Self {
            external_id: anime.mal_id.to_string(),
            title: anime.title,
            description: anime.synopsis,
            image_url: anime.images.jpg.image_url,
            content_type,
        }
    }
}

impl From<(Vn, ContentType)> for ContentSearchResult {
    fn from((vn, content_type): (Vn, ContentType)) -> Self {
        Self {
            external_id: vn.id,
            title: vn.title,
            description: vn.description,
            image_url: vn.image.url,
            content_type,
        }
    }
}
