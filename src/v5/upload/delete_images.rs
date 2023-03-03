//! Builder for deleting a set of uploaded images from an upload session.
//!
//! <https://api.mangadex.org/swagger.html#/Upload/delete-uploaded-session-files>
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
//! let session_id = Uuid::new_v4();
//! let session_file_id = Uuid::new_v4();
//! let res = client
//!     .upload()
//!     .delete_images()
//!     .session_id(&session_id)
//!     .add_session_file_id(session_file_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("delete images: {:?}", res);
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use derive_builder::Builder;
use mangadex_api_schema::{Endpoint, NoData};
use serde::Serialize;
use uuid::Uuid;

use crate::{HttpClientRef, Result};

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct DeleteImages<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub session_id: &'a Uuid,
    #[builder(setter(each = "add_session_file_id"))]
    pub session_file_ids: Vec<Uuid>,
}

// MangaDex takes an array for the request body rather than a traditional JSON body for this endpoint.
impl Endpoint for DeleteImages<'_> {
    type Query = ();
    type Body = Vec<Uuid>;
    type Response = Result<NoData>;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/upload/{}/batch", self.session_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::DELETE
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn body(&self) -> Option<&Vec<Uuid>> {
        Some(&self.session_file_ids)
    }
}

impl DeleteImages<'_> {
    pub async fn send(&self) -> Result<NoData> {
        #[cfg(not(feature = "multi-thread"))]
        {
            self.http_client.borrow().send_request(self).await?
        }
        #[cfg(feature = "multi-thread")]
        {
            self.http_client.lock().await.send_request(self).await?
        }
    }
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

    #[tokio::test]
    async fn delete_images_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let session_id = Uuid::new_v4();
        let session_file_id = Uuid::new_v4();
        let expected_body = json!([session_file_id]);
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("DELETE"))
            .and(path_regex(r"/upload/[0-9a-fA-F-]+/batch"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .upload()
            .delete_images()
            .session_id(&session_id)
            .add_session_file_id(session_file_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
