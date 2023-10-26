use mangadex_api_types::{ResultType, UserRole};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CheckTokenResponse {
    #[serde(default)]
    pub result: ResultType,
    pub is_authenticated: bool,
    pub roles: Vec<UserRole>,
    pub permissions: Vec<String>, // TODO: Deserialize the strings into `UserPermission` enum.
}
