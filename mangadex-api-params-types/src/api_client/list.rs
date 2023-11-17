#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::api_client::get::ListClientsBuilder;
use mangadex_api_types::{ApiClientState, ReferenceExpansionResource};

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct ApiClientListParam {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub state: Option<ApiClientState>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientListParam> for ListClientsBuilder {
    fn from(value: ApiClientListParam) -> Self {
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
        if let Some(name) = value.name {
            builder.name(name);
        }
        builder.includes(value.includes);
        builder
    }
}
