//! Builder for fetching a Manga's relations.
//!
//! This fetches the related Manga for the specified Manga such as spin-offs, doujinshis, and more.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-relation>
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
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .manga_id(manga_id)
//!     .relation()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("manga relation list: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaRelationListResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct ListMangaRelations {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/manga/{}/relation", manga_id),
    #[query] ListMangaRelations,
    #[flatten_result] MangaRelationListResponse,
    ListMangaRelationsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{MangaRelation, RelationshipType, ResponseType};

    #[tokio::test]
    async fn get_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": "0b92f446-4ee0-4c15-9e5c-6ae1211e785b",
                    "type": "manga_relation",
                    "attributes": {
                        "relation": "doujinshi",
                        "version": 1
                    },
                    "relationships": [
                        {
                            "id": "7944cc53-86e6-4135-898f-47c5c8d0647c",
                            "type": "manga"
                        }
                    ]
                },
                {
                    "id": "31b831b7-aac5-4797-b3eb-a41575cd4399",
                    "type": "manga_relation",
                    "attributes": {
                        "relation": "doujinshi",
                        "version": 1
                    },
                    "relationships": [
                        {
                            "id": "119327ab-9b32-4841-9068-02264c15e118",
                            "type": "manga"
                        }
                    ]
                },
                {
                    "id": "53815c02-b357-4e23-b8e7-0d6d114ea420",
                    "type": "manga_relation",
                    "attributes": {
                        "relation": "doujinshi",
                        "version": 1
                    },
                    "relationships": [
                        {
                            "id": "25e26436-7eb7-4505-8711-6e014bb16fd7",
                            "type": "manga"
                        }
                    ]
                },
                {
                    "id": "6958767b-54c5-4b4c-8f0f-579a36389f68",
                    "type": "manga_relation",
                    "attributes": {
                        "relation": "doujinshi",
                        "version": 1
                    },
                    "relationships": [
                        {
                            "id": "0736a46a-1a34-4411-b665-a1e45ebf54a9",
                            "type": "manga"
                        }
                    ]
                },
                {
                    "id": "b358b2f5-beab-484a-9daf-880ad6085225",
                    "type": "manga_relation",
                    "attributes": {
                        "relation": "spin_off",
                        "version": 1
                    },
                    "relationships": [
                        {
                            "id": "1e4deefe-9eb8-4183-837a-f24002adb318",
                            "type": "manga"
                        }
                    ]
                }
            ],
            "limit": 5,
            "offset": 0,
            "total": 5
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/relation"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .manga_id(manga_id)
            .relation()
            .get()
            .send()
            .await?;

        let related = &res.data[0];
        assert_eq!(res.response, ResponseType::Collection);
        assert_eq!(related.type_, RelationshipType::MangaRelation);
        assert_eq!(related.attributes.relation, MangaRelation::Doujinshi);
        assert_eq!(related.attributes.version, 1);
        assert_eq!(related.relationships[0].type_, RelationshipType::Manga);
        assert!(related.relationships[0].related.is_none());
        assert!(related.relationships[0].attributes.is_none());

        Ok(())
    }
}
