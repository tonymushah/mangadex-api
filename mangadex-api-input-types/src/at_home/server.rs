#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    rate_limit::Limited, v5::at_home::server::id::get::GetAtHomeServerBuilder, MangaDexClient,
    Result as MDResult,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::AtHomeServer;

use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct AtHomeServerParams {
    chapter_id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    force_port_443: Option<bool>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AtHomeServerParams {
    pub async fn send(self, client: &MangaDexClient) -> MDResult<Limited<AtHomeServer>> {
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
