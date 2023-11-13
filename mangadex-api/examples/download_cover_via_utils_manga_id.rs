use anyhow::Result;
use mangadex_api::MangaDexClient;
use std::{fs::File, io::Write};
use uuid::Uuid;

/// Download the [Kimi tte Watashi no Koto Suki Nandesho?](https://mangadex.org/title/f75c2845-0241-4e69-87c7-b93575b532dd/kimi-tte-watashi-no-koto-suki-nandesho) cover
///
/// For test... of course :3
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = String::from("test-outputs");
    let manga_id: Uuid = Uuid::parse_str("f75c2845-0241-4e69-87c7-b93575b532dd")?;
    let client: MangaDexClient = MangaDexClient::default();
    let (filename, bytes) = client
        .download()
        .cover()
        // you can use
        //
        // ```rust
        // .quality(CoverQuality::Size512)
        // ``` for 512
        // or
        // ```rust
        // .quality(CoverQuality::Size256)
        // ``` for 256
        .build()?
        .via_manga_id(manga_id)
        .await?;
    let bytes = bytes?;
    let mut file = File::create(format!("{}/{}", output_dir, filename))?;
    file.write_all(&bytes)?;
    println!("donwloaded :3");
    Ok(())
}
