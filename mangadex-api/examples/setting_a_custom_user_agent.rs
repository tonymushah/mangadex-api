use mangadex_api_schema::v5::RelatedAttributes;
use mangadex_api_types::{Language, ReferenceExpansionResource};
use std::time::Duration;
use url::Url;

use mangadex_api::MangaDexClient;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut default_header = HeaderMap::new();
    // We set your default header
    default_header.insert(USER_AGENT, HeaderValue::from_str("myApp v0")?);

    let reqwest_client = Client::builder()
        .default_headers(default_header)
        // I set some timeout here
        .timeout(Duration::from_secs(15))
        .build()?;
    let client = MangaDexClient::new(reqwest_client);

    // Do stuff your client :3
    // I keep pretty much the same code as the previous example

    let res = client
        .manga()
        .get()
        .title("Konsei ga")
        // We include the author data
        .include(ReferenceExpansionResource::Author)
        // We include the arstits data
        .include(ReferenceExpansionResource::Artist)
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

        // We generate the link that goes to the Mangadex page
        let title_link =
            Url::parse("https://mangadex.org/title/")?.join(manga.id.to_string().as_str())?;

        println!("\tLink: {title_link}");
        println!();
        index += 1;
    }

    Ok(())
}
