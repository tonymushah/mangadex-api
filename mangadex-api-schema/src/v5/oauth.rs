use mangadex_api_types::ApiClientProfile;
use serde::Deserialize;

use super::AuthTokens;

#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_expires_in: u32,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(default)]
    #[serde(alias = "not-before-policy")]
    pub not_before_policy: u32,
    pub session_state: uuid::Uuid,
    pub scope: String,
    pub client_type: ApiClientProfile,
}

impl From<OAuthTokenResponse> for AuthTokens {
    fn from(value: OAuthTokenResponse) -> Self {
        Self {
            session: value.access_token.to_owned(),
            refresh: value.refresh_token.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClientInfo {
    pub client_id: String,
    pub client_secret: String,
}
