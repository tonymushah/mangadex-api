//! This example will fetch manga with recently published chapters for the specified languages.
//! By default, this will only fetch English-translated manga.
//!
//! `latest_updates` returns manga with chapter translations matching each ISO 639-1 2-letter `LANGUAGE`.
//! If no `LANGUAGE` is provided, it defaults to "en", English.
//!
//! # Usage
//!
//! ```
//! latest_updates [OPTION] [LANGUAGE...]
//! ```
//!
//! ## Options
//!
//! -h, --help  Output a usage message and exit.
//! -p, --page  Specify the page of results. Default is 1.
//! -l, --limit Specify the maximum number of results to return. Default is 10.
//!
//! # Examples
//!
//! This example will return up to 20 manga with newly published English and Japanese chapters.
//!
//! ```
//! latest_updates --limit 20 en ja
//! ```

use std::collections::HashSet;
use std::iter::FromIterator;

use clap::Parser;
use uuid::Uuid;

use mangadex_api::types::{
    ChapterSortOrder, Language, OrderDirection, ReferenceExpansionResource, RelationshipType,
};
use mangadex_api::v5::MangaDexClient;

#[derive(Parser)]
#[clap(
    name = "MangaDex Latest Chapters Updates",
    about = "Fetch manga with recent chapter updates"
)]
struct Args {
    /// Space-separated ISO 639-1 2-letter language code representation.
    #[clap(default_value = "en")]
    languages: Vec<Language>,
    /// Start the results from the specified page number, starting from 1.
    #[clap(short, long, default_value = "1")]
    page: u32,
    /// Set the maximum number of results to return.
    #[clap(short, long, default_value = "10")]
    limit: u32,
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

/// This will get the latest manga with recently published chapters.
///
/// This does the following:
///
/// 1. Get chapters, sorted by the `publishAt` field.
/// 2. Extract the manga IDs.
/// 3. Search for manga using the extracted IDs.
async fn run(args: Args) -> anyhow::Result<()> {
    println!("The specified chapter languages are: {:?}", args.languages);

    let client = MangaDexClient::default();

    let chapters = client
        .chapter()
        .list()
        .offset(get_page_offset(args.page, args.limit))
        .limit(args.limit)
        .translated_languages(args.languages)
        .order(ChapterSortOrder::PublishAt(OrderDirection::Descending))
        .build()?
        .send()
        .await?;

    // `HashSet` is used over directly collecting into a `Vec` because checking for an item's
    // existence in a `Vec` gets more expensive the more it grows.
    let manga_ids: HashSet<Uuid> = chapters
        .data
        .iter()
        .filter_map(|chapter| {
            for r in &chapter.relationships {
                if r.type_ == RelationshipType::Manga {
                    return Some(r.id);
                }
            }
            None
        })
        .collect();
    let manga_ids = Vec::from_iter(manga_ids);

    let manga_list_res = client
        .manga()
        .list()
        // This isn't used but if this data were to be used in an application,
        // having the cover art UUIDs would make it convenient to fetch the images.
        .include(&ReferenceExpansionResource::CoverArt)
        .limit(manga_ids.len() as u32)
        .manga_ids(manga_ids)
        .build()?
        .send()
        .await?;

    // This step isn't necessary but extracting the manga ID and title makes it more readable in the output.
    let manga_list: Vec<(Uuid, String)> = manga_list_res
        .data
        .into_iter()
        .map(|manga| {
            (
                manga.id,
                manga
                    .attributes
                    .title
                    .get(&Language::English)
                    .unwrap_or(&"<NO ENGLISH TITLE>".to_string())
                    .clone(),
            )
        })
        .collect();

    println!("\n{:#?}", manga_list);

    Ok(())
}

/// Calculate the offset needed to view the page results.
fn get_page_offset(page: u32, limit: u32) -> u32 {
    if page == 0 || limit == 0 {
        return 0;
    }

    limit * (page - 1)
}
