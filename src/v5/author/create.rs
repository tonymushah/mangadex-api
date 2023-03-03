//! Builder for the author creation endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Author/post-author>
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
//! let author_id = Uuid::new_v4();
//! let res = client
//!     .author()
//!     .create()
//!     .name("Author Name")
//!     .version(1u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("author create: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use url::Url;

use crate::HttpClientRef;
use mangadex_api_schema::v5::{AuthorResponse, LocalizedString};

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
#[non_exhaustive]
pub struct CreateAuthor<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub biography: Option<LocalizedString>,
    /// <https://twitter.com>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub twitter: Option<Option<Url>>,
    /// <https://www.pixiv.net>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pixiv: Option<Option<Url>>,
    /// <https://www.melonbooks.co.jp>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub melon_book: Option<Option<Url>>,
    /// <https://www.fanbox.cc>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fan_box: Option<Option<Url>>,
    /// <https://booth.pm>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub booth: Option<Option<Url>>,
    /// <https://www.nicovideo.jp>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nico_video: Option<Option<Url>>,
    /// <https://skeb.jp>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub skeb: Option<Option<Url>>,
    /// <https://fantia.jp>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fantia: Option<Option<Url>>,
    /// <https://www.tumblr.com>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tumblr: Option<Option<Url>>,
    /// <https://www.youtube.com>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub youtube: Option<Option<Url>>,
    /// [https://weibo.cn/u/](https://weibo.cn)
    /// or
    /// [https://m.weibo.cn/u/](https://m.weibo.cn)
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub weibo: Option<Option<Url>>,
    /// <https://blog.naver.com/>
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub naver: Option<Option<Url>>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub website: Option<Option<Url>>,
    /// >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub version: Option<u32>,
}

endpoint! {
    POST ("/author"),
    #[body auth] CreateAuthor<'_>,
    #[flatten_result] AuthorResponse
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
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn create_author_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let author_id = Uuid::new_v4();
        let author_name: String = Name().fake();
        let author_biography: String = Sentence(1..2).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = json!({
            "name": author_name,
            "version": 1
        });
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
                    "version": 2,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                },
                "relationships": [],
            }
        });

        Mock::given(method("POST"))
            .and(path(r"/author"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .author()
            .create()
            .name(author_name.as_str())
            .version(1u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
