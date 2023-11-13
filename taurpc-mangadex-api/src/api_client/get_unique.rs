#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::api_client::id::get::GetClientBuilder;
use mangadex_api_types::ReferenceExpansionResource;
use uuid::Uuid;

#[taurpc::ipc_type]
pub struct ApiClientGetUniqueParams {
    pub client_id: Uuid,

    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientGetUniqueParams> for GetClientBuilder {
    fn from(value: ApiClientGetUniqueParams) -> Self {
        let mut builder = Self::default();
        builder.client_id(value.client_id);
        builder.includes(value.includes);
        builder
    }
}
