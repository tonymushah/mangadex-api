#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::custom_list::post::CreateCustomListBuilder, MangaDexClient};

use mangadex_api_types::CustomListVisibility;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CustomListCreateParam {
    pub name: String,
    #[serde(default)]
    pub visibility: Option<CustomListVisibility>,
    #[serde(default)]
    pub manga: Vec<Uuid>,
    #[serde(default)]
    pub version: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CustomListCreateParam> for CreateCustomListBuilder {
    fn from(value: CustomListCreateParam) -> Self {
        let mut builder = Self::default();
        builder.name(value.name);
        if let Some(visibility) = value.visibility {
            builder.visibility(visibility);
        }
        builder.manga(value.manga);
        if let Some(version) = value.version {
            builder.version(version);
        } else {
            builder.version(1_u32);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CustomListCreateParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::CustomListResponse {
        let builder: CreateCustomListBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
