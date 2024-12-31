use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct JikanResponse {
    data: Vec<Anime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anime {
    mal_id: u64,
    title: String,
    images: JikanImages,
    synopsis: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JikanImages {
    jpg: JikanJpg,
}

#[derive(Serialize, Deserialize, Debug)]
struct JikanJpg {
    image_url: String,
}

#[derive(Deserialize, Debug)]
struct VndbResponse {
    results: Vec<Vn>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vn {
    title: String,
    image: VndbImages,
    id: String,
    description: Option<String>,
} 

#[derive(Serialize, Deserialize, Debug)]
struct VndbImages {
    url: String,
}

#[tokio::main]
#[tauri::command]
pub async fn search_mal(query: &str) -> Result<Vec<Anime>, String> {
    let url = format!("https://api.jikan.moe/v4/anime?q={}", query);
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let json_response: JikanResponse = resp.json().await.map_err(|e| e.to_string())?;

    let anime: Vec<Anime> = json_response.data.iter().take(10).map(|anime_data| Anime {
        mal_id: anime_data.mal_id,
        title: anime_data.title.clone(),
        images: JikanImages {
            jpg: JikanJpg {
                image_url: anime_data.images.jpg.image_url.clone(),
            }
        },
        synopsis: anime_data.synopsis.clone(),
    }).collect();

    Ok(anime)
}

#[tokio::main]
#[tauri::command]
pub async fn search_vndb(query: &str) -> Result<Vec<Vn>, String> {
    let filters = vec!["search", "=", query];
    let fields = "title, image.url, description";

    let payload = serde_json::json!({
        "filters": filters,
        "fields": fields,
    });

    let client = reqwest::Client::new();
    let resp = client.post("https://api.vndb.org/kana/vn")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json_response: VndbResponse = resp.json().await.map_err(|e| e.to_string())?;

    let vns: Vec<Vn> = json_response.results.iter().take(10).map(|vn_data| Vn {
        title: vn_data.title.clone(),
        image: VndbImages {
            url: vn_data.image.url.clone(),
        },
        id: vn_data.id.clone(),
        description: vn_data.description.clone(),
    }).collect();

    Ok(vns)
}