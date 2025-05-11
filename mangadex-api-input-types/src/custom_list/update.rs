#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::custom_list::id::put::UpdateCustomListBuilder, MangaDexClient};

use mangadex_api_types::CustomListVisibility;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CustomListUpdateParams {
    pub list_id: Uuid,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub name: Option<String>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub visibility: Option<CustomListVisibility>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
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
    ) -> mangadex_api::Result<mangadex_api_schema::v5::CustomListData> {
        <UpdateCustomListBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
