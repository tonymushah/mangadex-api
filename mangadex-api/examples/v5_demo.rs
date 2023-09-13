use clap::Parser;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;

#[derive(Parser)]
#[clap(
    name = "MangaDex API v5 Demo",
    about = "Simple demo to make GET requests to the MangaDex API."
)]
struct Args {
    /// Search a list of manga.
    #[clap(long)]
    manga_search: Option<String>,
    /// View a single Manga.
    #[clap(long)]
    manga_view: Option<Uuid>,
    /// Get recent chapters for a Manga.
    #[clap(long)]
    manga_feed: Option<Uuid>,
    /// Get a random Manga.
    #[clap(long)]
    manga_random: bool,
    /// Get a MD@Home node URL.
    #[clap(long)]
    node: Option<Uuid>,
    /// Search for authors matching the provided name.
    #[clap(long)]
    author_search: Option<String>,
    /// Search a list of chapters belonging to a manga ID.
    #[clap(long)]
    chapter_search: Option<Uuid>,
    /// View a single chapter.
    #[clap(long)]
    chapter_view: Option<Uuid>,
    /// Search a list of cover art belonging to a manga ID.
    #[clap(long)]
    cover_search: Option<Uuid>,
    /// View a single cover.
    #[clap(long)]
    cover_view: Option<Uuid>,
    /// Get Manga volumes and chapters.
    #[clap(long)]
    manga_aggregate: Option<Uuid>,
    /// Search a list of scanlation groups matching the provided name.
    #[clap(long)]
    group_search: Option<String>,
    /// View a single scanlation group by its ID (UUID).
    #[clap(long)]
    group_view: Option<Uuid>,
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

    if let Some(title) = args.manga_search {
        let manga_builder = client.manga();

        let mut manga_list_builder = manga_builder.list();
        if !title.is_empty() {
            manga_list_builder = manga_list_builder.title(title.as_str());
        }
        let manga_list_builder = manga_list_builder.build()?;
        let manga_results = manga_list_builder.send().await?;

        println!("Manga results: {:#?}", manga_results);

        return Ok(());
    }

    if let Some(manga_id) = args.manga_view {
        let manga_view = client
            .manga()
            .get()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        println!("Manga view: {:#?}", manga_view);

        return Ok(());
    }

    if let Some(manga_id) = args.manga_feed {
        let manga_feed = client
            .manga()
            .feed()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        println!("Manga feed: {:#?}", manga_feed);

        return Ok(());
    }

    if args.manga_random {
        let manga = client.manga().random().build()?.send().await?;

        println!("Manga: {:#?}", manga);

        return Ok(());
    }

    if let Some(chapter_id) = args.node {
        let node_url = client
            .at_home()
            .server()
            .get()
            .chapter_id(chapter_id)
            .build()?
            .send()
            .await?;

        println!("MD@Home node URL: {:#?}", node_url);

        return Ok(());
    }

    if let Some(name) = args.author_search {
        let author_builder = client.author();

        let mut author_list_builder = author_builder.get();
        if !name.is_empty() {
            author_list_builder = author_list_builder.name(name.as_str());
        }
        let author_results = author_list_builder.build()?.send().await?;

        println!("Author results: {:#?}", author_results);

        return Ok(());
    }

    if let Some(manga_id) = args.chapter_search {
        let chapter_results = client
            .chapter()
            .get()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        println!("Chapter results: {:#?}", chapter_results);

        return Ok(());
    }

    if let Some(chapter_id) = args.chapter_view {
        let chapter_view = client
            .chapter()
            .id(chapter_id)
            .get()
            .build()?
            .send()
            .await?;

        println!("Chapter view: {:#?}", chapter_view);

        return Ok(());
    }

    if let Some(manga_id) = args.cover_search {
        let cover_results = client
            .cover()
            .list()
            .add_manga_id(&manga_id)
            .build()?
            .send()
            .await?;

        println!("Cover results: {:#?}", cover_results);

        return Ok(());
    }

    if let Some(cover_id) = args.cover_view {
        let cover_view = client
            .cover()
            .get()
            .cover_id(cover_id)
            .build()?
            .send()
            .await?;

        println!("Cover view: {:#?}", cover_view);

        return Ok(());
    }

    if let Some(manga_id) = args.manga_aggregate {
        let manga_aggregate = client
            .manga()
            .aggregate()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        println!("Manga aggregate: {:#?}", manga_aggregate);

        return Ok(());
    }

    if let Some(name) = args.group_search {
        let scanlation_group_builder = client.scanlation_group();

        let mut scanlation_group_list_builder = scanlation_group_builder.list();
        if !name.is_empty() {
            scanlation_group_list_builder = scanlation_group_list_builder.name(name.as_str());
        }
        let scanlation_group_results = scanlation_group_list_builder.build()?.send().await?;

        println!("Scanlation group results: {:#?}", scanlation_group_results);

        return Ok(());
    }

    if let Some(scanlation_group_id) = args.group_view {
        let scanlation_group_view = client
            .scanlation_group()
            .get()
            .group_id(scanlation_group_id)
            .build()?
            .send()
            .await?;

        println!("Scanlation group view: {:#?}", scanlation_group_view);

        return Ok(());
    }

    Ok(())
}
