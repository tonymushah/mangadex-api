#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::cover::cover_id::put::EditCoverBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::CoverData, Limited};

use mangadex_api_types::Language;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CoverEditParam {
    pub cover_or_manga_id: Uuid,

    /// 0-8 characters in length.
    pub volume: String,
    /// 0-512 characters in length.
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub description: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub locale: Option<Language>,
    /// >= 1
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CoverEditParam> for EditCoverBuilder {
    fn from(value: CoverEditParam) -> Self {
        let mut builder = Self::default();
        builder.cover_id(value.cover_or_manga_id);
        builder.volume(value.volume);
        if let Some(description) = value.description {
            builder.description(description);
        }
        if let Some(locale) = value.locale {
            builder.locale(locale);
        }
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CoverEditParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<CoverData>> {
        let builder: EditCoverBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
