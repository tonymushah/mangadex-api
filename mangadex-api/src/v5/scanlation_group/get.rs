//! Builder for the scanlation group list endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-search-group>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::MangaStatus;
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let group_res = client
//!     .scanlation_group()
//!     .list()
//!     .name("mangadex")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("groups: {:?}", group_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::GroupListResponse;
use mangadex_api_types::{GroupSortOrder, Language, ReferenceExpansionResource};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    default,
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct ListGroup {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[builder(setter(each = "add_group_id"))]
    #[serde(rename = "ids")]
    pub group_ids: Vec<Uuid>,
    pub name: Option<String>,
    /// Language the scanlation primarily translates or uploads works into.
    // The corresponding response body field returns an array so this seems likely to change to accept an array of languages.
    pub focused_language: Option<Language>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
    pub order: Option<GroupSortOrder>,
}

endpoint! {
    GET "/group",
    #[query] ListGroup,
    #[flatten_result] GroupListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{MangaDexDateTime, ResponseType};

    #[tokio::test]
    async fn list_scanlation_groups_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let group_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": group_id,
                    "type": "scanlation_group",
                    "attributes": {
                        "name": "Scanlation Group",
                        "altNames": [
                            {
                                "en": "Alternative Name"
                            }
                        ],
                        "website": "https://example.org",
                        "ircServer": null,
                        "ircChannel": null,
                        "discord": null,
                        "contactEmail": null,
                        "description": null,
                        "twitter": null,
                        "focusedLanguages": ["en"],
                        "locked": false,
                        "official": false,
                        "verified": false,
                        "inactive": false,
                        "publishDelay": "P6WT5M",
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/group"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .scanlation_group()
            .get()
            .limit(1u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let group = &res.data[0];
        assert_eq!(group.id, group_id);
        assert_eq!(group.attributes.name, "Scanlation Group");
        assert_eq!(
            group.attributes.website,
            Some("https://example.org".to_string())
        );
        assert_eq!(group.attributes.irc_server, None);
        assert_eq!(group.attributes.irc_channel, None);
        assert_eq!(group.attributes.discord, None);
        assert_eq!(group.attributes.contact_email, None);
        assert_eq!(group.attributes.description, None);
        assert!(group.attributes.twitter.is_none());
        assert!(!group.attributes.locked);
        assert_eq!(group.attributes.version, 1);
        assert_eq!(
            group.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            group.attributes.updated_at.to_string(),
            datetime.to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_scanlation_groups_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 400,
                "title": "Invalid limit",
                "detail": "Limit must be between 1 and 100"
            }]
        });

        Mock::given(method("GET"))
            .and(path("/group"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .scanlation_group()
            .get()
            .limit(0u32)
            .build()?
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Invalid limit".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Limit must be between 1 and 100".to_string())
            );
        }

        Ok(())
    }
}