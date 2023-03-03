//! Builder for deleting a Manga relation.
//!
//! This endpoint requires authentication.
//!
//! This removes the relationship between Manga.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/delete-manga-relation-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::types::MangaRelation;
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
//! let manga_id = Uuid::new_v4();
//! let target_manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .delete_relation()
//!     .manga_id(&manga_id)
//!     .relation_id(&target_manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("deleted manga relation: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::{HttpClientRef, Result};
use mangadex_api_schema::NoData;

#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct DeleteMangaRelation<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub manga_id: &'a Uuid,
    #[serde(skip)]
    pub relation_id: &'a Uuid,
}

endpoint! {
    DELETE ("/manga/{}/relation/{}", manga_id, relation_id),
    #[no_data auth] DeleteMangaRelation<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn delete_manga_relation_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let relation_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("DELETE"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/relation/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .delete_relation()
            .manga_id(&manga_id)
            .relation_id(&relation_id)
            .build()?
            .send()
            .await;

        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn delete_manga_relation_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let relation_id = Uuid::new_v4();
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

        Mock::given(method("DELETE"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/relation/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .delete_relation()
            .manga_id(&manga_id)
            .relation_id(&relation_id)
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
