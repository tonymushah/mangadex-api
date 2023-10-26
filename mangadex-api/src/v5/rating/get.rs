//! Builder for getting your Manga ratings.
//!
//! This endpoint requires authentication.
//!
//! <https://api.mangadex.org/swagger.html#/Rating/get-rating>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::{Password, Username};
//! use mangadex_api::v5::MangaDexClient;
//! use uuid::Uuid;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter2")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! // Official Test Manga ID.
//! let manga_id = Uuid::parse_str("f9c33607-9180-4ba6-b85c-e4b5faee7192")?;
//!
//! let res = client
//!     .rating()
//!     .get_your_manga_ratings()
//!     .manga_id(&manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("Response: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::RatingsResponse;

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
pub struct GetYourMangaRatings {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[builder(setter(each = "manga_id"))]
    pub manga: Vec<Uuid>,
}

endpoint! {
    GET "/rating",
    #[query auth] GetYourMangaRatings,
    #[flatten_result] RatingsResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn your_manga_ratings_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let response_body = json!({
            "result": "ok",
            "ratings": {
                manga_id.to_string(): {
                    "rating": 7,
                    "createdAt": "2021-12-27T08:47:37+00:00"
                }
            }
        });

        Mock::given(method("GET"))
            .and(path("/rating"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.rating().get().build()?.send().await?;

        assert_eq!(res.ratings.get(&manga_id).unwrap().rating, 7);

        Ok(())
    }

    #[tokio::test]
    async fn your_manga_ratings_requires_auth() -> anyhow::Result<()> {
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
                "status": 403,
                "title": "Forbidden",
                "detail": "You must be logged in to continue."
            }]
        });

        Mock::given(method("GET"))
            .and(path(r"/rating"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .rating()
            .get()
            .build()?
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            _ => panic!("unexpected error: {:#?}", res),
        }

        Ok(())
    }
}
