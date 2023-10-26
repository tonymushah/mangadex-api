//! Builder for the ping endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Infrastructure/get_ping>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let res = client
//!     .infrastructure()
//!     .ping()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("ping: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_types::error::{Error, Result};

/// Ping the server.
///
/// Makes a request to `GET /ping`.
// It doesn't make much sense to make this a builder pattern but for consistency, it is.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct Ping {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,
}

endpoint! {
    GET "/ping",
    #[no_data] Ping,
    #[no_send] Result<String>
}

impl Ping {
    pub async fn send(&self) -> Result<String> {
        #[cfg(not(feature = "multi-thread"))]
        let res = self
            .http_client
            .try_borrow()?
            .send_request_without_deserializing(self)
            .await?;
        #[cfg(feature = "multi-thread")]
        let res = self
            .http_client
            .lock()
            .await
            .send_request_without_deserializing(self)
            .await?;

        let response_body = res.text().await?;
        if response_body.as_str() == "pong" {
            return Ok(response_body);
        }

        Err(Error::PingError)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn ping_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = "pong";

        Mock::given(method("GET"))
            .and(path(r"/ping"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.ping().get().build()?.send().await?;

        assert_eq!(res, "pong".to_string());

        Ok(())
    }
}
