//! Builder for the chapter mark read endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Chapter/chapter-id-read>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
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
//! let chapter_id = Uuid::new_v4();
//! let chapter_read_res = client
//!     .chapter()
//!     .mark_read()
//!     .chapter_id(&chapter_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("chapter read: {:?}", chapter_read_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Mark a chapter as read for the current user.
///
/// Makes a request to `POST /chapter/{id}/read`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct MarkChapterRead<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(rename = "id", skip)]
    pub chapter_id: &'a Uuid,
}

endpoint! {
    POST ("/chapter/{}/read", chapter_id),
    #[no_data auth] MarkChapterRead<'_>,
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

    #[tokio::test]
    async fn mark_chapter_read_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let response_body = json!({"result": "ok"});

        Mock::given(method("POST"))
            .and(path_regex(r"/chapter/[0-9a-fA-F-]+/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .chapter()
            .mark_read()
            .chapter_id(&chapter_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
