//! Builder for committing an active upload session.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload/commit-upload-session>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api_types::Language;
//! use mangadex_api::v5::MangaDexClient;
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
//! let res = client
//!     .upload()
//!     .upload_session_id(session_id)
//!     .commit()
//!     .post()
//!     .volume(Some("1".to_string()))
//!     .chapter(Some("1".to_string()))
//!     .title(Some("Chapter Title".to_string()))
//!     .translated_language(Language::English)
//!     .send()
//!     .await?;
//!
//! println!("commit upload session: {:?}", res);
//! # Ok(())
//! # }
//! ```

use mangadex_api_schema::v5::ChapterData;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::error::{Error, Result};
use mangadex_api_types::{Language, MangaDexDateTime};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitUploadSession {
    /// This should never be set manually as this is only for internal use.
    #[serde(skip)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub session_id: Uuid,

    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    chapter_draft: ChapterDraft,
    /// Ordered list of Upload Session File IDs.
    ///
    /// Any uploaded files that are not included in this list will be deleted.
    pub page_order: Vec<Uuid>,
}

#[cfg_attr(feature = "deserializable-endpoint", derive(serde::Deserialize))]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDraft {
    /// Nullable
    pub volume: Option<String>,
    /// Nullable
    pub chapter: Option<String>,
    /// Nullable
    pub title: Option<String>,
    pub translated_language: Language,
    /// Must be a URL with "http(s)://".
    ///
    /// Nullable
    pub external_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<MangaDexDateTime>,
}

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
/// Custom request builder to handle nested struct.
#[derive(Debug, Serialize, Clone, Default)]
pub struct CommitUploadSessionBuilder {
    #[serde(skip)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: Option<HttpClientRef>,

    pub session_id: Option<Uuid>,
    /// Ordered list of Upload Session File IDs.
    pub page_order: Vec<Uuid>,

    /// Nullable
    pub volume: Option<String>,
    /// Nullable
    pub chapter: Option<String>,
    /// Nullable
    pub title: Option<String>,
    pub translated_language: Option<Language>,
    /// Must be a URL with "http(s)://".
    ///
    /// Nullable
    pub external_url: Option<Url>,
    pub publish_at: Option<MangaDexDateTime>,
}

impl CommitUploadSessionBuilder {
    pub fn new(http_client: HttpClientRef) -> Self {
        Self {
            http_client: Some(http_client),
            ..Default::default()
        }
    }

    #[doc(hidden)]
    pub fn http_client(mut self, http_client: HttpClientRef) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// Specify the upload session ID to commit.
    pub fn session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Specify the Upload Session File IDs to commit, ordered.
    pub fn page_order(mut self, page_order: Vec<Uuid>) -> Self {
        self.page_order = page_order;
        self
    }

    /// Add an Upload Session File ID to commit, adds to the end of the `pageOrder` list.
    pub fn add_page(mut self, page: Uuid) -> Self {
        self.page_order.push(page);
        self
    }

    /// Specify the volume the chapter belongs to.
    ///
    /// Nullable
    pub fn volume(mut self, volume: Option<String>) -> Self {
        self.volume = volume;
        self
    }

    /// Specify the chapter number the session is for.
    ///
    /// Nullable
    pub fn chapter(mut self, chapter: Option<String>) -> Self {
        self.chapter = chapter;
        self
    }

    /// Specify the title for the chapter.
    ///
    /// Nullable
    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    /// Specify the chapter number the session is for.
    ///
    /// Nullable
    pub fn translated_language(mut self, translated_language: Language) -> Self {
        self.translated_language = Some(translated_language);
        self
    }

    /// Specify the URL where the chapter can be found.
    ///
    /// Nullable
    ///
    /// This should not be used if chapter has images uploaded to MangaDex.
    pub fn external_url(mut self, external_url: Option<Url>) -> Self {
        self.external_url = external_url;
        self
    }

    /// Specify the date and time the chapter was originally published at.
    pub fn publish_at<DT: Into<MangaDexDateTime>>(mut self, publish_at: DT) -> Self {
        self.publish_at = Some(publish_at.into());
        self
    }

    /// Validate the field values. Use this before building.
    fn validate(&self) -> std::result::Result<(), String> {
        if self.session_id.is_none() {
            return Err("session_id cannot be None".to_string());
        }

        if self.translated_language.is_none() {
            return Err("translated_language cannot be None".to_string());
        }

        Ok(())
    }

    /// Finalize the changes to the request struct and return the new struct.
    pub fn build(&self) -> Result<CommitUploadSession> {
        if let Err(error) = self.validate() {
            return Err(Error::RequestBuilderError(error));
        }

        let session_id = self
            .session_id
            .ok_or(Error::RequestBuilderError(String::from(
                "session_id must be provided",
            )))?;
        let translated_language =
            self.translated_language
                .ok_or(Error::RequestBuilderError(String::from(
                    "translated_language must be provided",
                )))?;

        Ok(CommitUploadSession {
            http_client: self
                .http_client
                .to_owned()
                .ok_or(Error::RequestBuilderError(String::from(
                    "http_client must be provided",
                )))?,

            session_id,
            chapter_draft: ChapterDraft {
                volume: self.volume.to_owned(),
                chapter: self.chapter.to_owned(),
                title: self.title.to_owned(),
                translated_language,
                external_url: self.external_url.to_owned(),
                publish_at: self.publish_at,
            },
            page_order: self.page_order.to_owned(),
        })
    }
}

endpoint! {
    PUT ("/upload/{}/commit", session_id),
    #[body auth] CommitUploadSession,
    #[rate_limited] ChapterData,
    CommitUploadSessionBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Language, MangaDexDateTime, RelationshipType};

    #[tokio::test]
    async fn commit_upload_session_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let chapter_id = Uuid::new_v4();
        let uploader_id = Uuid::new_v4();
        let chapter_title: String = Name().fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = json!({
            "chapterDraft": {
                "volume": "1",
                "chapter": "2.5",
                "title": chapter_title,
                "translatedLanguage": "en",
                "externalUrl": null
            },
            "pageOrder": [
                session_file_id
            ]
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": chapter_id,
                "type": "chapter",
                "attributes": {
                    "title": chapter_title,
                    "volume": "1",
                    "chapter": "2.5",
                    "pages": 4,
                    "translatedLanguage": "en",
                    "uploader": uploader_id,
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                    "publishAt": datetime.to_string(),
                    "readableAt": datetime.to_string(),
                },
                "relationships": [],
            }

        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/upload/[0-9a-fA-F-]+/commit"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
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

        let res = mangadex_client
            .upload()
            .upload_session_id(session_id)
            .commit()
            .post()
            .volume(Some("1".to_string()))
            .chapter(Some("2.5".to_string()))
            .title(Some(chapter_title.clone()))
            .translated_language(Language::English)
            .page_order(vec![session_file_id])
            .send()
            .await?;

        let res = &res.data;

        assert_eq!(res.id, chapter_id);
        assert_eq!(res.type_, RelationshipType::Chapter);
        assert_eq!(res.attributes.title, chapter_title.clone());
        assert_eq!(res.attributes.volume, Some("1".to_string()));
        assert_eq!(res.attributes.chapter, Some("2.5".to_string()));
        assert_eq!(res.attributes.pages, 4);
        assert_eq!(res.attributes.translated_language, Language::English);
        assert_eq!(res.attributes.external_url, None);
        assert_eq!(res.attributes.version, 1);
        assert_eq!(res.attributes.created_at.to_string(), datetime.to_string());
        assert_eq!(
            res.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );
        assert_eq!(res.attributes.publish_at.to_string(), datetime.to_string());
        assert_eq!(res.attributes.readable_at.to_string(), datetime.to_string());

        Ok(())
    }
}
