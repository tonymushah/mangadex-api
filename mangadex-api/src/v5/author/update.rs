//! Builder for the author update endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Author/put-author-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api_types::{Password, Username};
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
//!     .update()
//!     .author_id(&author_id)
//!     .version(2u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("author update: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::{AuthorResponse, LocalizedString};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct UpdateAuthor {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub author_id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
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
    pub version: u32,
}

endpoint! {
    PUT ("/author/{}", author_id),
    #[body auth] UpdateAuthor,
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
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn update_author_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
            "website": "https://example.org/",
            "version": 2
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
                    "website": "https://example.org",
                    "version": 2,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/author/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .author()
            .update()
            .author_id(author_id)
            .website(Some(Url::parse("https://example.org").unwrap()))
            .version(2u32)
            .build()?
            .send()
            .await?;

        assert_eq!(
            res.data.attributes.website,
            Some(Url::parse("https://example.org").unwrap())
        );
        assert_eq!(res.data.attributes.version, 2);

        Ok(())
    }
}
