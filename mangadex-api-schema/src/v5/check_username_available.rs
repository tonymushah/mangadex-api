use serde::Deserialize;

use crate::FromResponse;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckUsernameAvailableResponse {
    pub available: bool,
}

impl FromResponse for CheckUsernameAvailableResponse {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}
