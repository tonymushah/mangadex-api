use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let random_manga = client.manga().random().get().send().await?;

    println!("{random_manga:?}");

    Ok(())
}
