// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod dashboard;
mod launcher;
mod library;
mod search;
mod settings;

use common::database::Database;

fn main() {
    let db = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            library::commands::add_to_library,
            library::commands::get_library,
            library::commands::delete_content,
            library::commands::get_episodes,
            search::commands::search_content,
            launcher::commands::launch_content,
            settings::commands::load_config,
            settings::commands::save_config,
            dashboard::commands::get_activity_stats,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
