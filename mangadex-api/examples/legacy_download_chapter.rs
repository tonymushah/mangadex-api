// Imports used for downloading the pages to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    // Yeah, i'm a [`100 girlfriend`](https://mangadex.org/title/efb4278c-a761-406b-9d69-19603c5e4c8b/the-100-girlfriends-who-really-really-really-really-really-love-you) simp and what! >:)
    let chapter_id = Uuid::parse_str("f2a09509-3c09-4371-a810-ecb99242bd90")?;

    let at_home = client
        .at_home()
        .server()
        .id(chapter_id)
        .get()
        .send()
        .await?;

    let http_client = reqwest::Client::new();

    // Original quality. Use `.data.attributes.data_saver` for smaller, compressed images.
    let page_filenames = &at_home.chapter.data;
    for filename in page_filenames {
        // If using the data-saver option, use "/data-saver/" instead of "/data/" in the URL.
        let page_url = at_home
            .base_url
            .join(&format!(
                "/{quality_mode}/{chapter_hash}/{page_filename}",
                quality_mode = "data",
                chapter_hash = at_home.chapter.hash,
                page_filename = filename
            ))
            .unwrap();

        let res = http_client.get(page_url).send().await?;
        // The data should be streamed rather than downloading the data all at once.
        let bytes = res.bytes().await?;

        // This is where you would download the file but for this example,
        // we're just printing the raw data.
        // let mut file = File::create(&filename)?;
        // let _ = file.write_all(&bytes);
        println!("Chunk: {bytes:?}");
    }

    Ok(())
}
