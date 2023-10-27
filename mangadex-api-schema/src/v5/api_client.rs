use mangadex_api_types::{ApiClientProfile, ApiClientState, MangaDexDateTime};
use serde::Deserialize;

/// General Api Client information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiClientAttributes {
    pub name: String,
    pub description: Option<String>,
    pub profile: ApiClientProfile,
    pub external_client_id: Option<String>,
    pub is_active: bool,
    pub state: ApiClientState,
    pub created_at: MangaDexDateTime,
    pub updated_at: MangaDexDateTime,
    pub version: u32,
}
