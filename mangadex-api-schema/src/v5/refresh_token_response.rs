use serde::{Deserialize, Serialize};

use crate::v5::AuthTokens;
use crate::FromResponse;

/// The response when refreshing the session JWT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResponse {
    pub token: AuthTokens,
    pub message: Option<String>,
}

impl FromResponse for RefreshTokenResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}
