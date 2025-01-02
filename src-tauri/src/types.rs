use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    Anime,
    Vn,
}

// API response object for anime
#[derive(Deserialize, Debug)]
pub struct JikanResponse {
    pub data: Vec<Anime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    pub mal_id: u64,
    pub title: String,
    pub images: JikanImages,
    pub synopsis: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JikanImages {
    pub jpg: JikanJpg,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JikanJpg {
    pub image_url: String,
}

// API response object for visual novels
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

// Intermediate unified search result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentSearchResult {
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub content_type: ContentType,
}

// Database storage type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageItem {
    pub id: u64,
    pub content_type: ContentType,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail_path: String,
    pub executable_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u64,
    pub content_id: u64,
    pub start_time: String,
    pub duration: u64,
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

impl From<ContentSearchResult> for StorageItem {
    fn from(result: ContentSearchResult) -> Self {
        Self {
            id: 0,
            content_type: result.content_type,
            external_id: result.external_id,
            name: result.title,
            description: result.description,
            thumbnail_path: result.image_url,
            executable_path: "placeholder".to_string(),
        }
    }
}

// Error handling
#[derive(Debug, Serialize)]
pub enum ApiError {
    FetchError(String),
    ParseError(String),
    DatabaseError(String),
    ConfigError(String),
    LaunchError(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::FetchError(error.to_string())
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(error: rusqlite::Error) -> Self {
        ApiError::DatabaseError(error.to_string())
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::FetchError(e) => write!(f, "Fetch error: {}", e),
            ApiError::ParseError(e) => write!(f, "Parse error: {}", e),
            ApiError::DatabaseError(e) => write!(f, "Database error: {}", e),
            ApiError::ConfigError(e) => write!(f, "Config error: {}", e),
            ApiError::LaunchError(e) => write!(f, "Launch error: {}", e),
        }
    }
}

// Result type alias for consistent error handling
pub type Result<T> = std::result::Result<T, ApiError>;
