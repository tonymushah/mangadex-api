//! Builder for deleting an uploaded image from an upload session.
//!
//! <https://api.mangadex.org/swagger.html#/Upload/delete-uploaded-session-file>
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
//! let session_id = Uuid::new_v4();
//! let session_file_id = Uuid::new_v4();
//! let res = client
//!     .upload()
//!     .delete_image()
//!     .session_id(&session_id)
//!     .session_file_id(&session_file_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("delete image: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::NoData;
use serde::Serialize;
use uuid::Uuid;

use mangadex_api_types::error::Result; 
use crate::HttpClientRef;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct DeleteImage<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub session_id: &'a Uuid,
    #[serde(skip)]
    pub session_file_id: &'a Uuid,
}

endpoint! {
    DELETE ("/upload/{}/{}", session_id, session_file_id),
    #[no_data auth] DeleteImage<'_>,
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
    async fn delete_image_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("DELETE"))
            .and(path_regex(r"/upload/[0-9a-fA-F-]+/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .upload()
            .delete_image()
            .session_id(&session_id)
            .session_file_id(&session_file_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
