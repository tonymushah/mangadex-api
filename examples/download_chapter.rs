//! This example will download a chapter's pages.
//!
//! Reference: <https://api.mangadex.org/docs/reading-chapter/>
//!
//! If you are downloading the pages, the output will match Tachiyomi's style of:
//!
//! ```
//! [uploader]_[volume] [chapter] - [title]
//! ```
//!
//! Here are some examples:
//!
//! - "Doki Fansubs_Vol.9 Ch.97 - Coffee Break"
//! - "Doki Fansubs_Ch.20 - Crunch Time"
//! - "Doki Fansubs & Holo_Ch.86"
//!
//! # Usage
//!
//! ```
//! download_chapter [OPTION] [CHAPTERID]
//! ```
//!
//! ## Options
//!
//! -h, --help
//!     Output a usage message and exit.
//!
//! --data-saver
//!     Use compressed images, which have smaller filesizes.
//!
//! -o, --download
//!     Specify the directory to save the pages to.
//!
//! # Examples
//!
//! This example will get the chapter page data for the official test manga.
//!
//! ```
//! download_chapter c84f0bdd-0936-4fc3-8a7d-9b24303df33e
//! ```
//!
//! This will download the manga covers to the local filesystem at the specified directory.
//!
//! ```
//! download_chapter --download ./ c84f0bdd-0936-4fc3-8a7d-9b24303df33e
//! ```

use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Parser;
use reqwest::Url;
use uuid::Uuid;

use mangadex_api::types::RelationshipType;
use mangadex_api::v5::MangaDexClient;
use mangadex_api::HttpClientRef;

#[derive(Parser, Debug)]
#[clap(
    name = "Manga Chapter Downloader",
    about = "Fetch the pages for a chapter."
)]
struct Args {
    /// Chapter UUID.
    #[clap()]
    chapter_id: Uuid,
    #[clap(long)]
    data_saver: bool,
    /// Location to save the cover art.
    #[clap(short, long = "download", parse(from_os_str))]
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

    let mut output = args.output.clone().unwrap_or_default();

    // Fetch chapter data to use for naming the output directory.
    let chapter = client
        .chapter()
        .get()
        .chapter_id(&args.chapter_id)
        .build()?
        .send()
        .await?;

    // Determine the uploader between the scanlation groups and users to prefix the chapter
    // directory with. Scanlation groups take priority over users.
    let mut scanlation_groups = Vec::new();
    for r in &chapter.data.relationships {
        if r.type_ == RelationshipType::ScanlationGroup {
            let group = client
                .scanlation_group()
                .get()
                .group_id(&r.id)
                .build()?
                .send()
                .await?;

            scanlation_groups.push(group.data.attributes.name);
        }
    }
    let mut users = Vec::new();
    if scanlation_groups.is_empty() {
        for r in &chapter.data.relationships {
            if r.type_ == RelationshipType::User {
                let user = client.user().get().user_id(&r.id).build()?.send().await?;

                users.push(user.data.attributes.username);
            }
        }
    }
    let uploader = if !scanlation_groups.is_empty() {
        scanlation_groups.join(" & ")
    } else {
        users.join(" & ")
    };

    let volume_number = match chapter.data.attributes.volume {
        Some(v) => format!("Vol.{} ", v),
        None => "".to_string(),
    };
    let chapter_number = match chapter.data.attributes.chapter {
        Some(c) => format!("Ch.{} ", c),
        None => "".to_string(),
    };
    let title_separator = if (volume_number.is_empty() && chapter_number.is_empty())
        || chapter.data.attributes.title.is_empty()
    {
        ""
    } else {
        "- "
    };

    output.push(format!(
        "{uploader}_{volume}{chapter}{separator}{title}",
        uploader = uploader,
        volume = volume_number,
        chapter = chapter_number,
        separator = title_separator,
        title = chapter.data.attributes.title
    ));

    // Only create the output directory if the user has specified that they want to download the
    // pages.
    if args.output.is_some() && !output.is_dir() {
        println!("Created {:?}", &output);
        create_dir(&output)?;
    }

    let at_home = client
        .at_home()
        .server()
        .chapter_id(&args.chapter_id)
        .build()?
        .send()
        .await?;

    let page_filenames = if !args.data_saver {
        at_home.chapter.data
    } else {
        at_home.chapter.data_saver
    };

    for (i, server_filename) in page_filenames.iter().enumerate() {
        let path = Path::new(server_filename);
        let ext = match path.extension() {
            Some(e) => format!(".{}", e.to_str().unwrap_or("")),
            None => "".to_string(),
        };
        // Match how Tachiyomi names the pages with left-padded (with zeroes) 3-digit numbers.
        // For example, "001.png".
        let filename = format!("{:03}{}", i + 1, ext);

        let page_url = at_home
            .base_url
            .join(&format!(
                "/{quality_mode}/{chapter_hash}/{page_filename}",
                quality_mode = if args.data_saver {
                    "data-saver"
                } else {
                    "data"
                },
                chapter_hash = at_home.chapter.hash,
                page_filename = server_filename
            ))
            .unwrap();

        if args.output.is_some() {
            print!("Downloading {}...", &filename);
            // The `print!()` macro is line-buffered so flushing it ensures the output is emitted
            // immediately.
            std::io::stdout().flush()?;

            download_file(
                client.get_http_client().clone(),
                &page_url,
                output.as_path(),
                &filename,
            )
            .await?;

            println!("done");
        } else {
            #[cfg(not(feature = "multi-thread"))]
            let page_res = client
                .get_http_client()
                .clone()
                .borrow()
                .client
                .get(page_url.clone())
                .send()
                .await?;
            #[cfg(feature = "multi-thread")]
            let page_res = client
                .get_http_client()
                .lock()
                .await
                .client
                .get(page_url.clone())
                .send()
                .await?
                .bytes()
                .await?;
            println!("{:?} - {:#?}", filename, page_res);
        }
    }

    Ok(())
}

/// Download the URL contents into the local filesystem.
async fn download_file(
    http_client: HttpClientRef,
    url: &Url,
    output: &Path,
    file_name: &str,
) -> anyhow::Result<()> {
    #[cfg(not(feature = "multi-thread"))]
    let image_bytes = http_client
        .borrow()
        .client
        .get(url.clone())
        .send()
        .await?
        .bytes()
        .await?;
    #[cfg(feature = "multi-thread")]
    let image_bytes = http_client
        .lock()
        .await
        .client
        .get(url.clone())
        .send()
        .await?
        .bytes()
        .await?;

    let mut file_buffer = File::create(output.join(file_name))?;
    let _ = file_buffer.write_all(&image_bytes)?;

    Ok(())
}
