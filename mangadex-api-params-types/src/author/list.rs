#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::author::get::ListAuthorBuilder;

use mangadex_api_types::{AuthorSortOrder, ReferenceExpansionResource};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct AuthorListParams {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub author_ids: Vec<Uuid>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub order: Option<AuthorSortOrder>,
    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AuthorListParams> for ListAuthorBuilder {
    fn from(value: AuthorListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.author_ids(value.author_ids);
        if let Some(name) = value.name {
            builder.name(name);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}
