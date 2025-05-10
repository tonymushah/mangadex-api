#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::author::id::get::GetAuthorBuilder, MangaDexClient};
use mangadex_api_types::ReferenceExpansionResource;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AuthorGetUniqueParam {
    id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AuthorGetUniqueParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::AuthorData> {
        let builder: GetAuthorBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AuthorGetUniqueParam> for GetAuthorBuilder {
    fn from(value: AuthorGetUniqueParam) -> Self {
        let mut builder = Self::default();
        builder.author_id(value.id);
        builder.includes(value.includes);
        builder
    }
}
