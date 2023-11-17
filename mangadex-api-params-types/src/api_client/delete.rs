#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::api_client::id::delete::DeleteClientBuilder;

use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct ApiClientDeleteParam {
    pub client_id: Uuid,
    #[serde(default)]
    pub version: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientDeleteParam> for DeleteClientBuilder {
    fn from(value: ApiClientDeleteParam) -> Self {
        let mut builder = Self::default();
        builder.client_id(value.client_id);
        builder.version(value.version);
        builder
    }
}
