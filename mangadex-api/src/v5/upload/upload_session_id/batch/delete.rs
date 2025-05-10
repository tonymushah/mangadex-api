//! Builder for deleting a set of uploaded images from an upload session.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload/delete-uploaded-session-files>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//! /*
//!
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let session_id = Uuid::new_v4();
//! let session_file_id = Uuid::new_v4();
//!
//! let res = client
//!     .upload()
//!     .upload_session_id(session_id)
//!     .batch()
//!     .delete()
//!     .add_session_file_id(session_file_id)
//!     .send()
//!     .await?;
//!
//! println!("delete images: {:?}", res);
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use crate::{rate_limit::Limited, traits::Endpoint};
use derive_builder::Builder;
use mangadex_api_schema::NoData;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use crate::Result;

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
pub struct DeleteImages {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub session_id: Uuid,
    #[builder(setter(each = "add_session_file_id"))]
    pub session_file_ids: Vec<Uuid>,
}

// MangaDex takes an array for the request body rather than a traditional JSON body for this endpoint.
impl Endpoint for DeleteImages {
    type Query = ();
    type Body = Vec<Uuid>;
    type Response = NoData;

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

impl DeleteImages {
    pub async fn send(&self) -> Result<Limited<NoData>> {
        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        {
            self.http_client
                .try_borrow()?
                .send_request_with_rate_limit(self)
                .await
        }
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        {
            self.http_client
                .lock()
                .await
                .send_request_with_rate_limit(self)
                .await
        }
        #[cfg(feature = "rw-multi-thread")]
        {
            self.http_client
                .read()
                .await
                .send_request_with_rate_limit(self)
                .await
        }
    }
}

builder_send! {
    #[builder] DeleteImagesBuilder,
    #[rate_limited] NoData
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
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .upload()
            .upload_session_id(session_id)
            .batch()
            .delete()
            .add_session_file_id(session_file_id)
            .send()
            .await?;

        Ok(())
    }
}
