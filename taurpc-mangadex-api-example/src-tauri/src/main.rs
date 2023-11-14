// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mangadex_api::MangaDexClient;
use taurpc_mangadex_api::{init_router, Author};

#[tokio::main]
async fn main() {
    let mangadex_client = MangaDexClient::default();
    tauri::Builder::default()
        .invoke_handler(taurpc::create_ipc_handler(
            <MangaDexClient as Author>::into_handler(mangadex_client),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
