#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::draft::id::get::GetMangaDraftBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct GetMangaDraftParams {
    pub manga_id: Uuid,

    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<GetMangaDraftParams> for GetMangaDraftBuilder {
    fn from(value: GetMangaDraftParams) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl GetMangaDraftParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::MangaResponse {
        let builder: GetMangaDraftBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
