// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Arc};

use mangadex_api::MangaDexClient;
use mangadex_api_types::{MangaDexDateTime, MangaSortOrder};
use rspc::{Error as RspcError, Router};
use tauri::{AppHandle, Manager};

use time::{Duration, OffsetDateTime};

struct Ctx {
    client: MangaDexClient,
    app_handle: Arc<tokio::sync::watch::Receiver<Option<AppHandle>>>,
}

impl TryFrom<&Ctx> for AppHandle {
    type Error = RspcError;
    fn try_from(value: &Ctx) -> Result<Self, Self::Error> {
        value
            .app_handle
            .clone()
            .borrow()
            .clone()
            .ok_or(RspcError::new(
                rspc::ErrorCode::NotFound,
                String::from("App Hanlde Not found"),
            ))
    }
}

#[tokio::main]
async fn main() {
    let (app_sender, app_receiver) = tokio::sync::watch::channel::<Option<AppHandle>>(None);

    let rec_app = Arc::new(app_receiver);
    let client = MangaDexClient::default();

    let router = Router::<Ctx>::new()
        .query("mdx-popular-titles", |t| {
            t.resolver(|ctx, _: ()| async move {
                let app: AppHandle = (&ctx).try_into()?;
                // Take the local date and put substract it with 30 days
                let created_at_since = OffsetDateTime::now_utc()
                    .checked_sub(Duration::days(30))
                    .ok_or(RspcError::new(
                        rspc::ErrorCode::InternalServerError,
                        String::from("Unable to fetch the created_at_since date"),
                    ))?;
                let created_at_since = MangaDexDateTime::new(&created_at_since);
                let result = ctx
                    .client
                    .manga()
                    .get()
                    .created_at_since(created_at_since)
                    .order(MangaSortOrder::FollowedCount(
                        mangadex_api_types::OrderDirection::Descending,
                    ))
                    .send()
                    .await
                    .map_err(|e| {
                        RspcError::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })?;
                result.data.iter().for_each(|i| {
                    let _ = app.emit_all(format!("mangadex-manga-{}", i.id).as_str(), i);
                });
                Ok(result)
            })
        })
        .build();
    #[cfg(debug_assertions)]
    router
        .export_ts(PathBuf::from("../src/lib/bindings.ts"))
        .unwrap();
    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(
            Arc::new(router),
            move || Ctx {
                client: client.clone(),
                app_handle: rec_app.clone(),
            },
        ))
        .setup(move |e| {
            app_sender.send(Some(e.handle())).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
