use mangadex_api_types::{ApiClientProfile, ApiClientState, MangaDexDateTime, ResultType};
use serde::Deserialize;

use crate::TypedAttributes;

/// General Api Client information.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiClientAttributes {
    pub name: String,
    pub description: Option<String>,
    pub profile: ApiClientProfile,
    pub external_client_id: Option<String>,
    pub is_active: bool,
    pub state: ApiClientState,
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub created_at: MangaDexDateTime,
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub updated_at: MangaDexDateTime,
    pub version: u32,
}

impl TypedAttributes for ApiClientAttributes {
    const TYPE_: mangadex_api_types::RelationshipType =
        mangadex_api_types::RelationshipType::ApiClient;
}

/// General Api Client information.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiClientSecret {
    #[serde(default)]
    pub result: ResultType,
    pub data: String,
}
