//! Builder for the followed manga endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-manga>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! let res = client
//!     .user()
//!     .followed_manga()
//!     .limit(1_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("followed manga: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaListResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", default)]
pub struct FollowedManga {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/user/follows/manga",
    #[query auth] FollowedManga,
    #[flatten_result] MangaListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{
        ContentRating, Demographic, Language, MangaDexDateTime, MangaStatus, ResponseType,
    };

    #[tokio::test]
    async fn get_followed_manga_fires_a_request_to_base_url_ungrouped() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": manga_id,
                    "type": "manga",
                    "attributes": {
                        "title": {
                            "en": manga_title
                        },
                        "altTitles": [],
                        "description": {},
                        "isLocked": false,
                        "links": null,
                        "originalLanguage": "ja",
                        "lastVolume": null,
                        "lastChapter": null,
                        "publicationDemographic": "shoujo",
                        "status": "ongoing",
                        "year": null,
                        "contentRating": "safe",
                        "chapterNumbersResetOnNewVolume": true,
                        "availableTranslatedLanguages": ["en"],
                        "tags": [],
                        "state": "published",
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                        "version": 1
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .user()
            .followed_manga()
            .limit(1_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let manga = &res.data[0];
        assert_eq!(manga.id, manga_id);
        assert_eq!(
            manga.attributes.title.get(&Language::English).unwrap(),
            &manga_title
        );
        assert!(manga.attributes.alt_titles.is_empty());
        assert!(manga.attributes.description.is_empty());
        assert!(!manga.attributes.is_locked);
        assert_eq!(manga.attributes.links, None);
        assert_eq!(manga.attributes.original_language, Language::Japanese);
        assert_eq!(manga.attributes.last_volume, None);
        assert_eq!(manga.attributes.last_chapter, None);
        assert_eq!(
            manga.attributes.publication_demographic.unwrap(),
            Demographic::Shoujo
        );
        assert_eq!(manga.attributes.status, MangaStatus::Ongoing);
        assert_eq!(manga.attributes.year, None);
        assert_eq!(
            manga.attributes.content_rating.unwrap(),
            ContentRating::Safe
        );
        assert!(manga.attributes.tags.is_empty());
        assert_eq!(
            manga.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            manga.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );
        assert_eq!(manga.attributes.version, 1);

        Ok(())
    }
}
