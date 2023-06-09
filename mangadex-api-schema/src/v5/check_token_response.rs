use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CheckTokenResponse {
    pub is_authenticated: bool,
    pub roles: Vec<String>, // TODO: Deserialize the strings into `UserRole` enum.
    pub permissions: Vec<String>, // TODO: Deserialize the strings into `UserPermission` enum.
}
