use serde::Deserialize;

use crate::v5::AuthTokens;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct LoginResponse {
    pub token: AuthTokens,
}
