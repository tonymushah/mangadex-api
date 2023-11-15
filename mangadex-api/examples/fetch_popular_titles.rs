use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::RelatedAttributes;
use mangadex_api_types::{
    Language, MangaDexDateTime, MangaSortOrder, OrderDirection, ReferenceExpansionResource,
};
use time::{Duration, OffsetDateTime};
use url::Url;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    // Take the local date and put substract it with 30 days
    let created_at_since = OffsetDateTime::now_utc()
        .checked_sub(Duration::days(30))
        .unwrap();

    let created_at_since = MangaDexDateTime::new(&created_at_since);

    let res = client
        .manga()
        .get()
        // We pick up all manga that has been created during these last 30 days
        .created_at_since(created_at_since)
        // Mangadex Popular Titles is ordered by followedCount descending
        .order(MangaSortOrder::FollowedCount(OrderDirection::Descending))
        // We include the author data
        .include(ReferenceExpansionResource::Author)
        // We include the arstits data
        .include(ReferenceExpansionResource::Artist)
        .send()
        .await?;

    // Fetch the manga statistics
    let stats = client
        .statistics()
        .manga()
        .get()
        .manga(res.data.iter().map(|d| d.id).collect::<Vec<Uuid>>())
        .send()
        .await?;

    let not_found = String::from("Not found");

    // Just a simple index :3
    let mut index = 1;

    for manga in res.data {
        // Find the English title
        let title = manga
            .attributes
            .title
            .get(&Language::English)
            .unwrap_or(&not_found);

        println!("{index} ~ {title}");

        // Find the author name
        let author = manga
            .find_first_relationships(mangadex_api_types::RelationshipType::Author)
            .and_then(|e| {
                e.attributes.clone().map(|rel| match rel {
                    RelatedAttributes::Author(a) => a.name,
                    _ => not_found.clone(),
                })
            })
            .unwrap_or(not_found.clone());

        println!("\tAuthor: {author}");

        // Find the author name
        let artist = manga
            .find_first_relationships(mangadex_api_types::RelationshipType::Artist)
            .and_then(|e| {
                e.attributes.clone().map(|rel| match rel {
                    RelatedAttributes::Author(a) => a.name,
                    _ => not_found.clone(),
                })
            })
            .unwrap_or(not_found.clone());

        if artist != author {
            println!("\tArtist: {artist}");
        }

        if let Some(stat) = stats.statistics.get(&manga.id) {
            if let Some(average) = stat.rating.average {
                println!("\tAverage: {average}");
            }
        }

        // We generate the link that goes to the Mangadex page
        let title_link =
            Url::parse("https://mangadex.org/title/")?.join(manga.id.to_string().as_str())?;

        println!("\tLink: {title_link}");
        println!();
        index += 1;
    }

    println!("Done :3");

    Ok(())
}
