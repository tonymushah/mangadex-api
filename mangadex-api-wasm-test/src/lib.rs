use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

async fn run() -> anyhow::Result<()> {
    let client = mangadex_api::MangaDexClient::default();
    let res = client.manga().get().send().await?;
    log::info!("{:#?}", res);
    {
        let res = client
            .manga()
            .id(client.manga().random().get().send().await?.data.id)
            .recommendation()
            .get()
            .send()
            .await?;
        log::info!("{:#?}", res);
    }
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init().unwrap();

    run().await.map_err(|d| d.to_string().into())
}
