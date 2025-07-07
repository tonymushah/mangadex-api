use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_results = client.manga().get().title("full metal").send().await?;

    println!("manga results = {manga_results:?}");
    Ok(())
}
