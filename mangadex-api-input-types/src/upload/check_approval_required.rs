#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::check_approval_required::post::CheckApprovalRequiredBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::upload_required_approval::UploadRequiredApproval, Limited};

use mangadex_api_types::Language;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UploadCheckApprovalRequiredParam {
    pub manga_id: Uuid,
    pub locale: Language,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UploadCheckApprovalRequiredParam> for CheckApprovalRequiredBuilder {
    fn from(value: UploadCheckApprovalRequiredParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.locale(value.locale);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UploadCheckApprovalRequiredParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<UploadRequiredApproval>> {
        <CheckApprovalRequiredBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
