// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod api_client;

fn main() {
   tauri::Builder::default()
     .invoke_handler(tauri::generate_handler![database::read_items, database::write_item, api_client::get_test])
     .run(tauri::generate_context!())
     .expect("error while running tauri application");
}

