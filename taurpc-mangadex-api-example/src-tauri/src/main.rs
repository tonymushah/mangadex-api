// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mangadex_api::MangaDexClient;
use taurpc_mangadex_api::init_router;

#[tokio::main]
async fn main() {
    let mangadex_client = MangaDexClient::default();
    let router = init_router(&mangadex_client);
    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
