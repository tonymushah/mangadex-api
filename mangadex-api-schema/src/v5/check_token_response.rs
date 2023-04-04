use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckTokenResponse {
    pub is_authenticated: bool,
    pub roles: Vec<String>, // TODO: Deserialize the strings into `UserRole` enum.
    pub permissions: Vec<String>, // TODO: Deserialize the strings into `UserPermission` enum.
}
