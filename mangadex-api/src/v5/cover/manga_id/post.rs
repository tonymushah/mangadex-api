//! Builder for uploading a new manga cover.
//!
//! <https://api.mangadex.org/swagger.html#/Cover/upload-cover>
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
//! use mangadex_api_types::{Language, Password, Username};
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
//! let file_bytes = vec![0];
//! let res = client
//!     .cover()
//!     .upload()
//!     .manga_id(&manga_id)
//!     .file(file_bytes)
//!     .locale(Language::English)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("upload cover: {:?}", res);
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;

use derive_builder::Builder;
use mangadex_api_schema::v5::CoverData;
use mangadex_api_schema::Endpoint;
use mangadex_api_schema::Limited;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::{error::Result, Language};

/// Upload a new cover for a manga.
///
/// This requires authentication.
///
/// Makes a request to `POST /cover/{id}`.
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
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct UploadCover {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    /// Manga **or** Cover ID.
    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    /// Image bytes.
    pub file: Cow<'static, [u8]>,
    /// Volume number the cover is associated with.
    ///
    /// * Nullable
    /// * <= 8 characters
    /// * Pattern: `^(0|[1-9]\\d*)((\\.\\d+){1,2})?[a-z]?$`
    #[builder(default)]
    pub volume: Option<String>,
    #[builder(default)]
    pub description: String,
    pub locale: Language,
}

// TODO: Come up with a way to generalize multipart form data for the `Endpoint` trait.
impl Endpoint for UploadCover {
    type Query = ();
    type Body = ();
    type Response = CoverData;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/cover/{}", self.manga_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn multipart(&self) -> Option<Form> {
        let part = Part::bytes(self.file.clone());
        let mut form = Form::new().part("file", part);

        if let Some(volume) = &self.volume {
            let volume_part = Part::text(volume.to_string());
            form = form.part("volume", volume_part);
        }

        form = form.part("description", Part::text(self.description.to_string()));

        form = form.part("locale", Part::text(self.locale.code2().to_string()));

        Some(form)
    }
}

impl UploadCover {
    pub async fn send(&self) -> Result<Limited<<Self as Endpoint>::Response>> {
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

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, header_exists, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Language, MangaDexDateTime};

    #[tokio::test]
    async fn upload_cover_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let file_bytes = vec![0_u8];
        let cover_id = Uuid::new_v4();
        let description: String = Sentence(1..3).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

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
                "relationships": [],
            },
        });

        Mock::given(method("POST"))
            .and(path_regex("/cover/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            // The "multipart/form-data; boundary=[boundary]" Content-Type value is dynamic and can't easily be validated.
            .and(header_exists("Content-Type"))
            .respond_with(
                ResponseTemplate::new(201)
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
            .cover(manga_id)
            .file(file_bytes)
            .locale(Language::English)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
