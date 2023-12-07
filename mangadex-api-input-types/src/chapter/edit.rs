#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::chapter::id::put::UpdateChapterBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::ChapterData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_types::Language;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChapterUpdateParams {
    pub chapter_id: Uuid,

    /// <= 255 characters in length.
    ///
    /// Nullable.
    #[serde(default)]
    pub title: Option<String>,
    /// Volume number.
    ///
    /// Nullable.
    #[serde(default)]
    pub volume: Option<String>,
    /// Chapter number.
    ///
    /// <= 8 characters in length.
    ///
    /// Nullable.
    #[serde(default)]
    pub chapter: Option<String>,
    #[serde(default)]
    pub translated_language: Option<Language>,
    #[serde(default)]
    pub groups: Vec<Uuid>,
    /// >= 1
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl ChapterUpdateParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<ChapterData>> {
        let builder: UpdateChapterBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ChapterUpdateParams> for UpdateChapterBuilder {
    fn from(value: ChapterUpdateParams) -> Self {
        let mut builder = Self::default();
        builder.chapter_id(value.chapter_id);
        if let Some(title) = value.title {
            builder.title(title);
        }
        if let Some(volume) = value.volume {
            builder.volume(volume);
        }
        builder.chapter(value.chapter);
        if let Some(translated_language) = value.translated_language {
            builder.translated_language(translated_language);
        }
        builder.groups(value.groups);
        builder.version(value.version);
        builder
    }
}
