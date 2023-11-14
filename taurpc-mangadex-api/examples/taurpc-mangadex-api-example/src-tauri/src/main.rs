// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mangadex_api::MangaDexClient;
use taurpc_mangadex_api::init_router;

fn main() {
    let mangadex_client = MangaDexClient::default();
    tauri::Builder::default()
        .invoke_handler(init_router(&mangadex_client).into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
