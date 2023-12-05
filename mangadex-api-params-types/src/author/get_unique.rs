#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::author::id::get::GetAuthorBuilder, MangaDexClient};
use mangadex_api_types::ReferenceExpansionResource;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct AuthorGetUniqueParam {
    id: Uuid,
    #[serde(default)]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AuthorGetUniqueParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::AuthorResponse {
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
