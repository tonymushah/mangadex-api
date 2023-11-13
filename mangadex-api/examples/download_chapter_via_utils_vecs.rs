use anyhow::Result;
use mangadex_api::{utils::download::chapter::DownloadMode, MangaDexClient};
/// used for file exporting
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

/// It's from this manga called [`The Grim Reaper Falls In Love With A Human`](https://mangadex.org/title/be2efc56-1669-4e42-9f27-3bd232bca8ea/the-grim-reaper-falls-in-love-with-a-human)
///
/// [Chapter 1 English](https://mangadex.org/chapter/2b4e39a5-fba0-4055-a176-8b7e19faacdb) by [`Kredim`](https://mangadex.org/group/0b870e54-c75f-4d2e-8068-c40f939135fd/kredim)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "your-output-dir";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("32b229f6-e9bf-41a0-9694-63c11191704c")?;
    let chapter_files = client
        // We use the download builder
        .download()
        // Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        // You also use `DownloadMode::Normal` if you want some the original quality
        //
        // Default : Normal
        .mode(DownloadMode::DataSaver)
        // Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint) if true
        //
        // Default : false
        .report(true)
        // Something that i don`t really know about
        //
        // More details at : https://api.mangadex.org/docs/retrieving-chapter/#basics
        .force_port_443(false)
        .build()?
        .download_element_vec()
        .await?;
    create_dir_all(format!("{}{}", output_dir, chapter_id))?;
    for (filename, bytes_) in chapter_files {
        if let Ok(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
        } else if let Err(e) = bytes_ {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
