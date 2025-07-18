//! Builder for the create or update Manga rating endpoint.
//!
//! This endpoint requires authentication.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Rating/post-rating-manga-id>
//!
//! # Examples
//!
//! ```rust
//! // use mangadex_api_types::{Password, Username};
//! use mangadex_api::v5::MangaDexClient;
//! use uuid::Uuid;
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
//!         .password(Password::parse("hunter2")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! // Official Test Manga ID.
//! let manga_id = Uuid::parse_str("f9c33607-9180-4ba6-b85c-e4b5faee7192")?;
//!
//! let res = client
//!     .rating()
//!     .manga_id(manga_id)
//!     .post()
//!     .rating(9)
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
use crate::Result;
use mangadex_api_schema::NoData;

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
pub struct CreateUpdateMangaRating {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    /// `[ 1 .. 10 ]`.
    ///
    /// Numbers below `1` will be set at `1` and numbers above `10` will be set at `10`.
    pub rating: u8,
}

impl CreateUpdateMangaRating {
    pub async fn send(&mut self) -> Result<NoData> {
        self.rating = self.rating.clamp(1, 10);

        let res = self.http_client.read().await.send_request(self).await??;

        Ok(res)
    }
}

endpoint! {
    POST ("/rating/{}", manga_id),
    #[body auth] CreateUpdateMangaRating,
    #[no_send] Result<NoData>
}

builder_send! {
    #[builder] CreateUpdateMangaRatingBuilder,
    NoData
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn create_update_manga_rating_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({
            "rating": 9
        });

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/rating/[0-9a-fA-F-]+"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _res = mangadex_client
            .rating()
            .manga_id(manga_id)
            .post()
            .rating(9)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_update_manga_rating_sets_rating_below_min_to_1() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({
            "rating": 1
        });

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/rating/[0-9a-fA-F-]+"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _res = mangadex_client
            .rating()
            .manga_id(manga_id)
            .post()
            .rating(0)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_update_manga_rating_sets_rating_above_max_to_10() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({
            "rating": 10
        });

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/rating/[0-9a-fA-F-]+"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _res = mangadex_client
            .rating()
            .manga_id(manga_id)
            .post()
            .rating(11)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_update_manga_rating_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
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

        Mock::given(method("POST"))
            .and(path_regex(r"/rating/[0-9a-fA-F-]+"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .rating()
            .manga_id(manga_id)
            .post()
            .rating(7)
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            _ => panic!("unexpected error: {res:#?}"),
        }

        Ok(())
    }
}
