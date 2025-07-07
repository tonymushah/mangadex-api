//! This example will get mappings of legacy IDs to the new UUIDs.
//!
//! # Usage
//!
//! ```
//! map_legacy_ids [OPTION] [ID...]
//! ```
//!
//! ## Options
//!
//! -h, --help  Output a usage message and exit.
//! -t, --type  Specify the type of IDs that should be mapped.
//!             Available options are:
//!                 - chapter
//!                 - group
//!                 - manga
//!                 - tag
//!
//! # Examples
//!
//! This example will return up the new UUIDs for the legacy manga IDs 18803 and 1001.
//!
//! ```
//! map_legacy_ids -t manga 18803 1001
//! ```

use clap::Parser;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;
use mangadex_api_types::LegacyMappingType;

#[derive(Parser)]
#[clap(
    name = "MangaDex Legacy ID Mapping",
    about = "Get the new UUIDs from the legacy IDs"
)]
struct Args {
    /// Space-separated list of numerical IDs.
    #[clap()]
    ids: Vec<u32>,
    /// Legacy mapping type.
    #[clap(short, long, default_value = "manga")]
    r#type: LegacyMappingType,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        use std::process;
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// This will get the new UUIDs from the legacy IDs.
async fn run(args: Args) -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let mapped_ids = client
        .legacy()
        .mapping()
        .post()
        .map_type(args.r#type)
        .ids(args.ids)
        .build()?
        .send()
        .await?;

    let ids: Vec<(u32, Uuid)> = mapped_ids
        .data
        .iter()
        .map(|id_map| (id_map.attributes.legacy_id, id_map.attributes.new_id))
        .collect();

    println!("{ids:#?}");

    Ok(())
}
