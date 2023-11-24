//! Builder for uploading images to the upload session.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload/put-upload-session-file>
//!
//! Currently, there is a maximum of 10 files per request.
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
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
//! let file1_bytes = vec![0];
//! let file2_bytes = vec![1];
//! let res = client
//!     .upload()
//!     .upload_session_id(session_id)
//!     .post()
//!     .add_file(file1_bytes.into())
//!     .add_file(file2_bytes.into())
//!     .send()
//!     .await?;
//!
//! println!("upload images: {:?}", res);
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use derive_builder::Builder;
use mangadex_api_schema::Endpoint;
use mangadex_api_schema::{v5::UploadSessionFileDataObject, Limited};
use mangadex_api_types::error::Result;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

#[derive(Clone, Debug)]
pub struct UploadImage {
    pub filename: String,
    pub data: Vec<u8>,
}

impl TryFrom<PathBuf> for UploadImage {
    type Error = std::io::Error;
    fn try_from(value: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        if !value.is_file()
            || !value
                .extension()
                .is_some_and(|e| ["jpg", "jpeg", "png", "gif"].iter().any(|a| e == *a))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                String::from("The given path might not be a file or an image"),
            ));
        }
        let filename = String::from(value.as_path().file_name().and_then(|e| e.to_str()).ok_or(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                String::from("Can't parse the filename"),
            ),
        )?);
        let buf = {
            let mut data = Vec::<u8>::new();
            let mut buf_reader = BufReader::new(File::open(value)?);
            buf_reader.read_to_end(&mut data)?;
            data
        };
        Ok(Self {
            filename,
            data: buf,
        })
    }
}

impl TryFrom<&PathBuf> for UploadImage {
    type Error = std::io::Error;
    fn try_from(value: &PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        <Self as TryFrom<PathBuf>>::try_from(value.clone())
    }
}

/// Upload images to the upload session.
///
/// This requires authentication.
///
/// Makes a request to `POST /upload/{id}`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(getset::Getters, getset::Setters)
)]
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct UploadImages {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub session_id: Uuid,

    /// Image bytes.
    #[builder(setter(each = "add_file"))]
    #[serde(skip_serializing)]
    pub files: Vec<UploadImage>,
}

// TODO: Come up with a way to generalize multipart form data for the `Endpoint` trait.
impl Endpoint for UploadImages {
    type Query = ();
    type Body = ();
    type Response = UploadSessionFileDataObject;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("/upload/{}", self.session_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn require_auth(&self) -> bool {
        true
    }

    fn multipart(&self) -> Option<Form> {
        let mut form = Form::new();

        for (count, file) in self.files.iter().enumerate() {
            let part = Part::bytes(file.data.clone()).file_name(file.filename.clone());
            form = form.part(format!("file{count}"), part);
        }

        Some(form)
    }
}

impl UploadImages {
    pub async fn send(&self) -> Result<Limited<UploadSessionFileDataObject>> {
        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        let res = self
            .http_client
            .try_borrow()?
            .send_request_with_rate_limit(self)
            .await?;
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        let res = self
            .http_client
            .lock()
            .await
            .send_request_with_rate_limit(self)
            .await?;
        #[cfg(feature = "rw-multi-thread")]
        let res = self
            .http_client
            .read()
            .await
            .send_request_with_rate_limit(self)
            .await?;

        Ok(res)
    }
}

builder_send! {
    #[builder] UploadImagesBuilder,
    #[rate_limited] UploadSessionFileDataObject
}

#[cfg(test)]
mod tests {
    use fake::faker::filesystem::en::MimeType;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, header_exists, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::upload::upload_session_id::post::UploadImage;
    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn upload_images_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let file_bytes = vec![0_u8];
        let session_id = Uuid::new_v4();
        let mime_type: String = MimeType().fake();
        let response_body = json!({
            "result": "ok",
            "errors": [],
            "data": [
                {
                    "id": session_id,
                    "type": "upload_session_file",
                    "attributes": {
                        "originalFileName": "p01.jpg",
                        "fileHash": "e199c7d73af7a58e8a4d0263f03db660",
                        "fileSize": 0,
                        "mimeType": mime_type,
                        "source": "local",
                        "version": 1,
                    },
                    "relationships": []
                }
            ],
        });

        Mock::given(method("POST"))
            .and(path_regex("/upload/[0-9a-fA-F-]+"))
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
            .upload_session_id(session_id)
            .post()
            .add_file(UploadImage {
                filename: String::from("p01.jpg"),
                data: file_bytes,
            })
            .send()
            .await?;

        Ok(())
    }
}
