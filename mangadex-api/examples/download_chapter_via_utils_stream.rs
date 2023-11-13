use anyhow::Result;
use mangadex_api::{utils::download::chapter::DownloadMode, MangaDexClient};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tokio::pin;
use tokio_stream::StreamExt;

/// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
///
/// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "./test-outputs";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
    create_dir_all(format!("{}/{}", output_dir, chapter_id))?;
    let download = client
        // We use the download builder
        .download()
        // Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        // You also use `DownloadMode::Normal` if you want some the original quality
        //
        // Default : Normal
        .mode(DownloadMode::DataSaver)
        // Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/04-chapter/retrieving-chapter/) if true
        //
        // Default : false
        .report(true)
        // Something that i don`t really know about
        //
        // More details at : https://api.mangadex.org/docs/04-chapter/retrieving-chapter/
        .force_port_443(false)
        .build()?;
    let chapter_files = download.download_stream().await?;
    // `pin!` Required for iteration
    pin!(chapter_files);
    while let Some((data, index, total)) = chapter_files.next().await {
        let (filename, bytes_) = data;
        // Prin the progression in the standart output
        println!("{index} / {total} : {filename} ");
        if let Ok(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}/{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
            println!("downloaded");
        } else if let Err(e) = bytes_ {
            eprintln!("{e}");
        }
    }
    Ok(())
}
