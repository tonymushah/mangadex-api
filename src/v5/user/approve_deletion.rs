//! Builder for the account-activation endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/User/get-account-activate-code>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let code = Uuid::new_v4();
//! let res = client
//!     .user()
//!     .approve_deletion()
//!     .code(&code)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("deletion approval: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Approve the deletion of a user.
///
/// Makes a request to `POST /user/delete/{code}`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct ApproveUserDeletion<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub code: &'a Uuid,
}

endpoint! {
    POST ("/user/delete/{}", code),
    #[no_data] ApproveUserDeletion<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn approve_deletion_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let code = Uuid::new_v4();
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/user/delete/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .user()
            .approve_deletion()
            .code(&code)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
