use mangadex_api::v5::schema::RelatedAttributes;
use mangadex_api::v5::MangaDexClient;
// use mangadex_api_types_rust::{ReferenceExpansionResource, RelationshipType};
use mangadex_api_types::{ReferenceExpansionResource, RelationshipType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_results = client
        .manga()
        .get()
        .title("full metal")
        .include(&ReferenceExpansionResource::Author)
        .send()
        .await?;

    println!("manga results = {:?}", manga_results);

    let authors = manga_results.data.iter().filter_map(|manga| {
        manga
            .relationships
            .iter()
            .find(|&rel| rel.type_ == RelationshipType::Author)
    });

    for author in authors {
        if let Some(RelatedAttributes::Author(author_attributes)) = &author.attributes {
            println!("{} - {}", author.id, author_attributes.name);
        }
    }
    Ok(())
}
