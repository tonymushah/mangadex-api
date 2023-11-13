// Imports used for downloading the cover to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;
use mangadex_api::CDN_URL;
// use mangadex_api_types_rust::RelationshipType;
use mangadex_api_types::RelationshipType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_id = Uuid::new_v4();
    let manga = client.manga().id(manga_id).get().send().await?;

    let cover_id = manga
        .data
        .relationships
        .iter()
        .find(|related| related.type_ == RelationshipType::CoverArt)
        .expect("no cover art found for manga")
        .id;
    let cover = client.cover().cover_id(cover_id).get().send().await?;

    // This uses the best quality image.
    // To use smaller, thumbnail-sized images, append any of the following:
    //
    // - .512.jpg
    // - .256.jpg
    //
    // For example, "https://uploads.mangadex.org/covers/8f3e1818-a015-491d-bd81-3addc4d7d56a/4113e972-d228-4172-a885-cb30baffff97.jpg.512.jpg"
    let cover_url = Url::parse(&format!(
        "{}/covers/{}/{}",
        CDN_URL, manga_id, cover.data.attributes.file_name
    ))
    .unwrap();

    let http_client = reqwest::Client::new();

    let res = http_client.get(cover_url).send().await?;
    // The data should be streamed rather than downloading the data all at once.
    let bytes = res.bytes().await?;

    // This is where you would download the file but for this example, we're just printing the raw data.
    // let mut file = File::create(&filename)?;
    // let _ = file.write_all(&bytes);
    println!("Chunk: {:?}", bytes);
    Ok(())
}
