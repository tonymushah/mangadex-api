//! Builder for the create or update Manga rating endpoint.
//!
//! This endpoint requires authentication.
//!
//! <https://api.mangadex.org/swagger.html#/Rating/post-rating-manga-id>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::types::{Password, Username};
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
//!     .upsert_for_manga()
//!     .manga_id(&manga_id)
//!     .rating(9)
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

use crate::{HttpClientRef, Result};
use mangadex_api_schema::NoData;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct CreateUpdateMangaRating<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub manga_id: &'a Uuid,

    /// `[ 1 .. 10 ]`.
    ///
    /// Numbers below `1` will be set at `1` and numbers above `10` will be set at `10`.
    pub rating: u8,
}

impl CreateUpdateMangaRating<'_> {
    pub async fn send(&mut self) -> Result<NoData> {
        if self.rating < 1 {
            self.rating = 1;
        } else if self.rating > 10 {
            self.rating = 10;
        }

        #[cfg(not(feature = "multi-thread"))]
        let res = self.http_client.borrow().send_request(self).await??;
        #[cfg(feature = "multi-thread")]
        let res = self.http_client.lock().await.send_request(self).await??;

        Ok(res)
    }
}

endpoint! {
    POST ("/rating/{}", manga_id),
    #[body auth] CreateUpdateMangaRating<'_>,
    #[no_send] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn create_update_manga_rating_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
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
            .upsert_for_manga()
            .manga_id(&manga_id)
            .rating(9)
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_update_manga_rating_sets_rating_below_min_to_1() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
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
            .upsert_for_manga()
            .manga_id(&manga_id)
            .rating(0)
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_update_manga_rating_sets_rating_above_max_to_10() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
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
            .upsert_for_manga()
            .manga_id(&manga_id)
            .rating(11)
            .build()?
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
            .upsert_for_manga()
            .manga_id(&manga_id)
            .rating(7)
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
