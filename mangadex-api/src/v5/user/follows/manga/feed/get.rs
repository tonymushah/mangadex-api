//! Builder for the user manga feed endpoint to get a list of new chapters for a user's manga.
//!
//! <https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let res = client
//!     .user()
//!     .follows()
//!     .manga()
//!     .feed()
//!     .get()
//!     .limit(1u32)
//!     .send()
//!     .await?;
//!
//! println!("Manga feed: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::ChapterListResponse;
use mangadex_api_types::{
    ContentRating, IncludeExternalUrl, IncludeFuturePages, IncludeFuturePublishAt,
    IncludeFutureUpdates, Language, MangaDexDateTime, MangaFeedSortOrder,
    ReferenceExpansionResource,
};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(
    feature = "custom_list_v2",
    deprecated(
        since = "3.0.0-rc.1",
        note = "After the introduction of the Subscription system, this endpoint will be removed in v3"
    )
)]
pub struct GetFollowedMangaFeed {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[builder(setter(each = "add_translated_language"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translated_language: Vec<Language>,
    #[builder(setter(each = "add_original_language"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub original_language: Vec<Language>,
    #[builder(setter(each = "exclude_original_language"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_original_language: Vec<Language>,
    #[builder(setter(each = "add_content_rating"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    #[builder(setter(each = "excluded_group"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    #[builder(setter(each = "excluded_uploader"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_uploaders: Vec<Uuid>,
    /// Flag to include future chapter updates in the results.
    ///
    /// Default: `IncludeFutureUpdates::Include` (1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_updates: Option<IncludeFutureUpdates>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at_since: Option<MangaDexDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<MangaFeedSortOrder>,
    #[builder(setter(each = "include"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<ReferenceExpansionResource>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_empty_pages: Option<IncludeFuturePages>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_url: Option<IncludeExternalUrl>,
}

endpoint! {
    GET "/user/follows/manga/feed",
    #[query auth] GetFollowedMangaFeed,
    #[flatten_result] ChapterListResponse,
    GetFollowedMangaFeedBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_followed_manga_feed_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let chapter_id = Uuid::new_v4();
        let uploader_id = Uuid::new_v4();
        let chapter_title: String = Name().fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": chapter_id,
                    "type": "chapter",
                    "attributes": {
                        "title": chapter_title,
                        "volume": "1",
                        "chapter": "1.5",
                        "pages": 4,
                        "translatedLanguage": "en",
                        "uploader": uploader_id,
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                        "publishAt": datetime.to_string(),
                        "readableAt": datetime.to_string(),
                    },
                    "relationships": [],
                },
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/manga/feed"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .user()
            .follows()
            .manga()
            .feed()
            .get()
            .limit(1u32)
            .send()
            .await?;

        Ok(())
    }
}
