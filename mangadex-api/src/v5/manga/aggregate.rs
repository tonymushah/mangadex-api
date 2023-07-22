//! Builder for the manga aggregate endpoint to get volumes and chapters.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get_manga__id__aggregate>
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
//! let manga_res = client
//!     .manga()
//!     .aggregate()
//!     .manga_id(&manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("manga aggregate: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaAggregateResponse;
use mangadex_api_types::Language;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
#[non_exhaustive]
pub struct GetMangaAggregate {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    #[builder(setter(each = "add_language"), default)]
    pub translated_language: Vec<Language>,
    #[builder(setter(each = "add_group"), default)]
    pub groups: Vec<Uuid>,
}

endpoint! {
    GET ("/manga/{}/aggregate", manga_id),
    #[query] GetMangaAggregate,
    #[flatten_result] MangaAggregateResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn manga_aggregate_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let chapter_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "volumes": {
                "1": {
                    "volume": "1",
                    "count": 2,
                    "chapters": {
                        "26": {
                            "chapter": "26",
                            "id": chapter_id,
                            "others": [],
                            "count": 2
                        }
                    }
                }
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/aggregate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .aggregate()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        assert_eq!(res.volumes.len(), 1);

        let volume = &res.volumes[0];
        assert_eq!(volume.volume, "1");
        assert_eq!(volume.count, 2);

        assert_eq!(volume.chapters.len(), 1);
        let chapter = &volume.chapters[0];
        assert_eq!(chapter.chapter, "26");
        assert_eq!(chapter.id, chapter_id);
        assert_eq!(chapter.count, 2);

        Ok(())
    }

    #[tokio::test]
    async fn manga_aggregate_handles_array_volumes() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "volumes": []
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/aggregate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .aggregate()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        assert_eq!(res.volumes.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn manga_aggregate_handles_array_chapters() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let chapter_id = Uuid::parse_str("01defbd2-ab44-4672-9236-ff71b82774e8").unwrap();
        let response_body = json!({
            "result": "ok",
            "volumes": {
                "0": {
                    "volume": "0",
                    "count": 1,
                    "chapters": [
                        {
                            "chapter": "0",
                            "id": "01defbd2-ab44-4672-9236-ff71b82774e8",
                            "others": [],
                            "count": 1
                        }
                    ]
                },
                "1": {
                    "volume": "1",
                    "count": 9,
                    "chapters": {
                        "1": {
                            "chapter": "1",
                            "id": "7729dc89-38bd-4e90-a566-e63558d28cfd",
                            "others": [
                                "c30e8966-cef8-46c6-bc31-f24f6827bf84",
                                "b5ced190-b89f-43e9-b915-7fc0e5596c71"
                            ],
                            "count": 3
                        },
                        "2": {
                            "chapter": "2",
                            "id": "68f1bce2-1ea3-4100-b8a5-3c182507906c",
                            "others": [
                                "6fd96344-741d-4a6c-9ff0-0a177f459cbc",
                                "b1ae77a4-d587-4fd7-8f32-d180c619d9bf"
                            ],
                            "count": 3
                        },
                        "3": {
                            "chapter": "3",
                            "id": "8ea939ec-02fb-4a4a-836c-ae09871c3354",
                            "others": [
                                "cbe758e6-ea2e-4cb4-a621-df9e06181519",
                                "f08c1906-b2a6-45fa-a26a-4cc3b1cf1dab"
                            ],
                            "count": 3
                        }
                    }
                },
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/aggregate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .aggregate()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        assert_eq!(res.volumes.len(), 2);

        let volume = &res.volumes[0];
        assert_eq!(volume.volume, "0");
        assert_eq!(volume.count, 1);
        assert_eq!(volume.chapters.len(), 1);
        let chapter = &volume.chapters[0];
        assert_eq!(chapter.chapter, "0");
        assert_eq!(chapter.id, chapter_id);
        assert_eq!(chapter.count, 1);

        Ok(())
    }
}
