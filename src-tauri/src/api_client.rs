use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct JikanResponse {
    data: Vec<Anime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
    mal_id: u64,
    title: String,
    images: Images
}

#[derive(Serialize, Deserialize, Debug)]
struct Images {
    jpg: Jpg,
}

#[derive(Serialize, Deserialize, Debug)]
struct Jpg {
    image_url: String,
}

#[tokio::main]
#[tauri::command]
pub async fn get_test(query: &str) -> Result<Vec<Anime>, String> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let jikan_response: JikanResponse = resp.json().await.map_err(|e| e.to_string())?;

    let anime: Vec<Anime> = jikan_response.data.iter().take(10).map(|anime_data| Anime {
        mal_id: anime_data.mal_id,
        title: anime_data.title.clone(),
        images: Images {
            jpg: Jpg {
                image_url: anime_data.images.jpg.image_url.clone(),
            }
        }
    }).collect();

    Ok(anime)
}