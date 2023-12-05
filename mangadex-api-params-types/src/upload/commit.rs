#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::commit::post::CommitUploadSessionBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::ChapterData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_types::{Language, MangaDexDateTime};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CommitUploadSessionParam {
    pub session_id: Uuid,
    /// Ordered list of Upload Session File IDs.
    pub page_order: Vec<Uuid>,

    /// Nullable
    #[serde(default)]
    pub volume: Option<String>,
    /// Nullable
    #[serde(default)]
    pub chapter: Option<String>,
    /// Nullable
    pub title: Option<String>,

    pub translated_language: Language,
    /// Must be a URL with "http(s)://".
    ///
    /// Nullable
    #[serde(default)]
    pub external_url: Option<Url>,
    #[serde(default)]
    pub publish_at: Option<MangaDexDateTime>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CommitUploadSessionParam> for CommitUploadSessionBuilder {
    fn from(value: CommitUploadSessionParam) -> Self {
        let mut builder = Self::default()
            .page_order(value.page_order)
            .volume(value.volume)
            .chapter(value.chapter)
            .title(value.title)
            .translated_language(value.translated_language)
            .external_url(value.external_url);
        if let Some(publish_at) = value.publish_at {
            builder = builder.publish_at(publish_at);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CommitUploadSessionParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<ChapterData>> {
        <CommitUploadSessionBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
