#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::draft::get::ListMangaDraftsBuilder, MangaDexClient};

use mangadex_api_types::{MangaDraftsSortOrder, MangaState, ReferenceExpansionResource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct MangaDraftsParams {
    pub limit: Option<u32>,
    /// >= 0
    pub offset: Option<u32>,
    pub state: Option<MangaState>,
    pub order: Option<MangaDraftsSortOrder>,
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaDraftsParams> for ListMangaDraftsBuilder {
    fn from(value: MangaDraftsParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        if let Some(state) = value.state {
            builder.state(state);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaDraftsParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::MangaListResponse {
        <ListMangaDraftsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
