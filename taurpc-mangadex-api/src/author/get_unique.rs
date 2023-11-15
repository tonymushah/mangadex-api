#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::author::id::get::GetAuthorBuilder;
use mangadex_api_types::ReferenceExpansionResource;
use uuid::Uuid;

#[taurpc::ipc_type]
pub struct AuthorGetUniqueParam {
    id: Uuid,
    #[serde(default)]
    includes: Vec<ReferenceExpansionResource>,
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
