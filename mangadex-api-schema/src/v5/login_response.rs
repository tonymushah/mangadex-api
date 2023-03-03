use serde::Deserialize;

use crate::v5::AuthTokens;
use crate::FromResponse;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: AuthTokens,
}

impl FromResponse for LoginResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}
