//! Builder for committing an active upload session.
//!
//! <https://api.mangadex.org/swagger.html#/Upload/commit-upload-session>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api_types::Language;
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
//! let res = client
//!     .upload()
//!     .commit_session()
//!     .session_id(&session_id)
//!     .volume(Some("1"))
//!     .chapter(Some("1"))
//!     .title(Some("Chapter Title"))
//!     .translated_language(Language::English)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("commit upload session: {:?}", res);
//! # Ok(())
//! # }
//! ```

use mangadex_api_schema::v5::ChapterObject;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::error::{Error, Result};
use mangadex_api_types::{Language, MangaDexDateTime};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitUploadSession<'a> {
    /// This should never be set manually as this is only for internal use.
    #[serde(skip)]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub session_id: &'a Uuid,

    chapter_draft: ChapterDraft<'a>,
    /// Ordered list of Upload Session File IDs.
    ///
    /// Any uploaded files that are not included in this list will be deleted.
    pub page_order: Vec<Uuid>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDraft<'a> {
    /// Nullable
    pub volume: Option<&'a str>,
    /// Nullable
    pub chapter: Option<&'a str>,
    /// Nullable
    pub title: Option<&'a str>,
    pub translated_language: Language,
    /// Must be a URL with "http(s)://".
    ///
    /// Nullable
    pub external_url: Option<&'a Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<MangaDexDateTime>,
}

/// Custom request builder to handle nested struct.
#[derive(Debug, Serialize, Clone, Default)]
pub struct CommitUploadSessionBuilder<'a> {
    #[serde(skip)]
    pub(crate) http_client: HttpClientRef,

    pub session_id: Option<&'a Uuid>,
    /// Ordered list of Upload Session File IDs.
    pub page_order: Vec<Uuid>,

    /// Nullable
    pub volume: Option<&'a str>,
    /// Nullable
    pub chapter: Option<&'a str>,
    /// Nullable
    pub title: Option<&'a str>,
    pub translated_language: Option<Language>,
    /// Must be a URL with "http(s)://".
    ///
    /// Nullable
    pub external_url: Option<&'a Url>,
    pub publish_at: Option<MangaDexDateTime>,
}

impl<'a> CommitUploadSessionBuilder<'a> {
    pub fn new(http_client: HttpClientRef) -> Self {
        Self {
            http_client,
            ..Default::default()
        }
    }

    /// Specify the upload session ID to commit.
    pub fn session_id(mut self, session_id: &'a Uuid) -> Self {
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
    pub fn volume(mut self, volume: Option<&'a str>) -> Self {
        self.volume = volume;
        self
    }

    /// Specify the chapter number the session is for.
    ///
    /// Nullable
    pub fn chapter(mut self, chapter: Option<&'a str>) -> Self {
        self.chapter = chapter;
        self
    }

    /// Specify the title for the chapter.
    ///
    /// Nullable
    pub fn title(mut self, title: Option<&'a str>) -> Self {
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
    pub fn external_url(mut self, external_url: Option<&'a Url>) -> Self {
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
    pub fn build(self) -> Result<CommitUploadSession<'a>> {
        if let Err(error) = self.validate() {
            return Err(Error::RequestBuilderError(error));
        }

        let session_id = self.session_id.unwrap();
        let translated_language = self.translated_language.unwrap();

        Ok(CommitUploadSession {
            http_client: self.http_client,

            session_id,
            chapter_draft: ChapterDraft {
                volume: self.volume,
                chapter: self.chapter,
                title: self.title,
                translated_language,
                external_url: self.external_url,
                publish_at: self.publish_at,
            },
            page_order: self.page_order,
        })
    }
}

endpoint! {
    PUT ("/upload/{}/commit", session_id),
    #[body auth] CommitUploadSession<'_>,
    ChapterObject
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
        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/upload/[0-9a-fA-F-]+/commit"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .upload()
            .commit_session()
            .session_id(&session_id)
            .volume(Some("1"))
            .chapter(Some("2.5"))
            .title(Some(&chapter_title))
            .translated_language(Language::English)
            .page_order(vec![session_file_id])
            .build()?
            .send()
            .await?;

        assert_eq!(res.id, chapter_id);
        assert_eq!(res.type_, RelationshipType::Chapter);
        assert_eq!(res.attributes.title, chapter_title);
        assert_eq!(res.attributes.volume, Some("1".to_string()));
        assert_eq!(res.attributes.chapter, Some("2.5".to_string()));
        assert_eq!(res.attributes.pages, 4);
        assert_eq!(res.attributes.translated_language, Language::English);
        assert_eq!(res.attributes.uploader, Some(uploader_id));
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
