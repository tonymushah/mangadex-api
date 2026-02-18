#[cfg(feature = "mangadex-api-resolver")]
use std::{
    fs::File,
    io::BufReader,
};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{rate_limit::Limited, Result};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::cover::manga_id::post::UploadCoverBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::CoverData;

use crate::PathBuf;
use mangadex_api_types::Language;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CoverUploadParam {
    pub manga_id: Uuid,
    pub file: PathBuf,
    /// Volume number the cover is associated with.
    ///
    /// * Nullable
    /// * <= 8 characters
    /// * Pattern: `^(0|[1-9]\\d*)((\\.\\d+){1,2})?[a-z]?$`
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub volume: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub description: String,
    pub locale: Language,
}

#[cfg(feature = "mangadex-api-resolver")]
impl TryFrom<CoverUploadParam> for UploadCoverBuilder {
    type Error = std::io::Error;
    fn try_from(value: CoverUploadParam) -> Result<Self, Self::Error> {
        let file = File::open(value.file)?;
        let mut buffread = BufReader::new(file);
        let mut file_content = std::io::Cursor::new(Vec::<u8>::new());
        std::io::copy(&mut buffread, &mut file_content)?;

        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.file(file_content.into_inner());
        if let Some(volume) = value.volume {
            builder.volume(volume);
        }
        builder.description(value.description);
        builder.locale(value.locale);
        Ok(builder)
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CoverUploadParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<CoverData>> {
        let builder: UploadCoverBuilder = self.try_into()?;
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
