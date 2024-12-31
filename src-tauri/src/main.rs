// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod api;

fn main() {
   tauri::Builder::default()
     .invoke_handler(tauri::generate_handler![database::read_items, database::write_item, api::api_client::search_mal, api::api_client::search_vndb])
     .run(tauri::generate_context!())
     .expect("error while running tauri application");
}
