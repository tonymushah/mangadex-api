use serde::Deserialize;

use crate::v5::AuthTokens;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LoginResponse {
    pub token: AuthTokens,
}
