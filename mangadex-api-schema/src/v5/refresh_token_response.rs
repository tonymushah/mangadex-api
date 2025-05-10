use serde::Deserialize;

use crate::v5::AuthTokens;

/// The response when refreshing the session JWT.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RefreshTokenResponse {
    pub token: AuthTokens,
    pub message: Option<String>,
}
