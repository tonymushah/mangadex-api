//! This example will fetch manga cover art data.
//!
//! A `MANGA` is a UUID.
//!
//! # Usage
//!
//! ```
//! manga_covers [OPTION] [MANGA...]
//! ```
//!
//! ## Options
//!
//! -h, --help     Output a usage message and exit.
//! -o, --download Specify the directory to save the images to.
//!
//! # Examples
//!
//! This example will get the cover art data for the official test manga.
//!
//! ```
//! manga_covers f9c33607-9180-4ba6-b85c-e4b5faee7192
//! ```
//!
//! This will download the manga covers to the local filesystem at the specified directory.
//!
//! ```
//! manga_covers --download ./ f9c33607-9180-4ba6-b85c-e4b5faee7192
//! ```

use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Parser;
use reqwest::Url;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;
use mangadex_api::{HttpClientRef, CDN_URL};
use mangadex_api_types::{Language, RelationshipType};

#[derive(Parser)]
#[clap(
    name = "Manga Cover Art Downloader",
    about = "Fetch the cover art for manga."
)]
struct Args {
    /// Manga UUID.
    #[clap()]
    manga_ids: Vec<Uuid>,
    /// Location to save the cover art.
    #[clap(short, long = "download")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        use std::process;
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    if args.output.is_some() && !args.output.as_ref().unwrap().is_dir() {
        let _ = create_dir(args.output.as_ref().unwrap());
    }

    let manga_covers = client
        .cover()
        .get()
        .manga_ids(args.manga_ids)
        .build()?
        .send()
        .await?;

    for cover_data in manga_covers.data {
        // There should only be one relationship with type "manga".
        let manga_id = cover_data
            .relationships
            .iter()
            .filter_map(|r| {
                if r.type_ == RelationshipType::Manga {
                    Some(r.id)
                } else {
                    None
                }
            })
            .collect::<Vec<Uuid>>()[0];
        let manga = client.manga().id(manga_id).get().build()?.send().await?;

        let no_title = "<NO ENGLISH TITLE>".to_string();
        let manga_title = manga
            .data
            .attributes
            .title
            .get(&Language::English)
            .unwrap_or(&no_title);

        let file_name = cover_data.attributes.file_name;

        let cover_url = Url::parse(&format!(
            "{base_url}/covers/{manga_id}/{cover_file_name}",
            base_url = CDN_URL,
            manga_id = manga_id,
            cover_file_name = file_name
        ))?;

        if args.output.is_some() {
            println!(
                "Downloading cover for manga {:?}, cover file name {:?}",
                manga_title, &file_name
            );

            download_file(
                client.get_http_client().clone(),
                cover_url,
                args.output.as_ref().unwrap(),
                &file_name,
            )
            .await?;
        } else {
            println!("{:?} - {:?}", manga_title, file_name);
        }
    }

    Ok(())
}

/// Download the URL contents into the local filesystem.
async fn download_file(
    http_client: HttpClientRef,
    url: Url,
    output: &Path,
    file_name: &str,
) -> anyhow::Result<()> {
    let image_bytes = http_client
        .read()
        .await
        .client
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;

    let mut file_buffer = File::create(output.join(file_name))?;
    file_buffer.write_all(&image_bytes)?;

    Ok(())
}
