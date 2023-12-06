#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::at_home::server::id::get::GetAtHomeServerBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::AtHomeServer, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct AtHomeServerParams {
    chapter_id: Uuid,
    #[serde(default)]
    force_port_443: Option<bool>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AtHomeServerParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<AtHomeServer>> {
        let builder: GetAtHomeServerBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AtHomeServerParams> for GetAtHomeServerBuilder {
    fn from(value: AtHomeServerParams) -> Self {
        let mut builder = Self::default();
        builder.chapter_id(value.chapter_id);
        builder.force_port_443(value.force_port_443.unwrap_or_default());
        builder
    }
}
