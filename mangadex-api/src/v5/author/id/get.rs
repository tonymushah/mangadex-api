//! Builder for the author view endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Author/get-author-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let author_id = Uuid::new_v4();
//! let res = client
//!     .author()
//!     .id(author_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("author view: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::AuthorObject;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::ReferenceExpansionResource;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
pub struct GetAuthor {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub author_id: Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/author/{}", author_id),
    #[query] GetAuthor,
    #[flatten_result] crate::Result<AuthorObject>,
    GetAuthorBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_author_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let author_id = Uuid::new_v4();
        let author_name: String = Name().fake();
        let author_biography: String = Sentence(1..2).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": author_id,
                "type": "author",
                "attributes": {
                    "name": author_name,
                    "imageUrl": "",
                    "biography": {
                        "en": author_biography,
                    },
                    "twitter": null,
                    "pixiv": null,
                    "melonBook": null,
                    "fanBox": null,
                    "booth": null,
                    "nicoVideo": null,
                    "skeb": null,
                    "fantia": null,
                    "tumblr": null,
                    "youtube": null,
                    "weibo": null,
                    "naver": null,
                    "website": null,
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/author/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client.author().id(author_id).get().send().await?;

        Ok(())
    }
}
