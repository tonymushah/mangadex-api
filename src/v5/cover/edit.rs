//! Builder for the edit cover endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Cover/edit-cover>
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
//! let cover_id = Uuid::new_v4();
//! let res = client
//!     .cover()
//!     .edit()
//!     .cover_id(&cover_id)
//!     .volume(Some("1"))
//!     .version(2_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("edit: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CoverResponse;
use mangadex_api_types::Language;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
#[non_exhaustive]
pub struct EditCover<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    /// Manga **or** Cover ID.
    #[serde(skip)]
    pub cover_id: &'a Uuid,

    /// 0-8 characters in length.
    pub volume: Option<Option<&'a str>>,
    /// 0-512 characters in length.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<Option<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub locale: Option<Option<Language>>,
    /// >= 1
    pub version: u32,
}

endpoint! {
    PUT ("/cover/{}", cover_id),
    #[body auth] EditCover<'_>,
    #[flatten_result] CoverResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn edit_cover_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let cover_id = Uuid::new_v4();
        let description: String = Sentence(1..3).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = json!({
            "volume": "1",
            "version": 2
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": cover_id,
                "type": "cover_art",
                "attributes": {
                    "volume": "1",
                    "fileName": "1.jpg",
                    "description": description,
                    "locale": "en",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                },
                "relationships": []
            }
        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/cover/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .cover()
            .edit()
            .cover_id(&cover_id)
            .volume(Some("1"))
            .version(2_u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
