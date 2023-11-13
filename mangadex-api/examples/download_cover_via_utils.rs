use anyhow::Result;
use mangadex_api::MangaDexClient;
use std::{fs::File, io::Write};
use uuid::Uuid;

/// Download the volume 2 cover of [Lycoris Recoil](https://mangadex.org/title/9c21fbcd-e22e-4e6d-8258-7d580df9fc45/lycoris-recoil)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = String::from("test-outputs");
    let cover_id: Uuid = Uuid::parse_str("b5040c5d-7fd3-4b17-af5f-c96805c83aa4")?;
    let client: MangaDexClient = MangaDexClient::default();
    let (filename, bytes_) = client
        .download()
        .cover()
        .build()?
        .via_cover_id(cover_id)
        .await?;
    let bytes_ = bytes_?;
    let mut file = File::create(format!("{}/{}", output_dir, filename))?;
    file.write_all(&bytes_)?;
    println!("downloaded :3");
    Ok(())
}
