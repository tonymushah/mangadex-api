use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::RelatedAttributes;
use mangadex_api_types::{
    Language, MangaDexDateTime, MangaSortOrder, OrderDirection, ReferenceExpansionResource,
};
use time::{Duration, OffsetDateTime};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let created_at_since = OffsetDateTime::now_utc()
        .checked_sub(Duration::days(30))
        .unwrap();

    let created_at_since = MangaDexDateTime::new(&created_at_since);

    let res = client
        .manga()
        .get()
        .created_at_since(created_at_since)
        .order(MangaSortOrder::FollowedCount(OrderDirection::Descending))
        .include(ReferenceExpansionResource::Author)
        .include(ReferenceExpansionResource::Artist)
        .send()
        .await?;

    let not_found = String::from("Not found");

    for manga in res.data {
        // Find the English title
        let title = manga
            .attributes
            .title
            .get(&Language::English)
            .unwrap_or(&not_found);

        println!("{title}");

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

        let title_link =
            Url::parse("https://mangadex.org/title/")?.join(manga.id.to_string().as_str())?;

        println!("\tLink: {title_link}");
        println!();
    }

    Ok(())
}
