#[cfg(feature = "mangadex-api-resolver")]
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::cover::manga_id::post::UploadCoverBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::CoverData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_types::Language;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct CoverUploadParam {
    pub manga_id: Uuid,
    pub file: PathBuf,
    /// Volume number the cover is associated with.
    ///
    /// * Nullable
    /// * <= 8 characters
    /// * Pattern: `^(0|[1-9]\\d*)((\\.\\d+){1,2})?[a-z]?$`
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub description: String,
    pub locale: Language,
}

#[cfg(feature = "mangadex-api-resolver")]
impl TryFrom<CoverUploadParam> for UploadCoverBuilder {
    type Error = std::io::Error;
    fn try_from(value: CoverUploadParam) -> Result<Self, Self::Error> {
        let file = File::open(value.file)?;
        let mut buffread = BufReader::new(file);
        let mut file_content: Vec<u8> = vec![1, 0];
        buffread.read_exact(&mut file_content)?;

        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.file(file_content);
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
