#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::custom_list::id::put::UpdateCustomListBuilder, MangaDexClient};

use mangadex_api_types::CustomListVisibility;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CustomListUpdateParams {
    pub list_id: Uuid,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub visibility: Option<CustomListVisibility>,

    #[serde(default)]
    pub manga: Vec<Uuid>,

    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CustomListUpdateParams> for UpdateCustomListBuilder {
    fn from(value: CustomListUpdateParams) -> Self {
        let mut builder = Self::default();
        builder.list_id(value.list_id);
        if let Some(name) = value.name {
            builder.name(name);
        }
        if let Some(visibility) = value.visibility {
            builder.visibility(visibility);
        }
        builder.manga(value.manga);
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CustomListUpdateParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::CustomListResponse {
        <UpdateCustomListBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
