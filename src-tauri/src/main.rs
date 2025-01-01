// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::Database;

mod commands;
mod database;
mod types;

fn main() {
    let db = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::search_content,
            commands::add_to_library,
            commands::get_from_library,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
