use serde::{Deserialize, Serialize};

use crate::FromResponse;

#[derive(Debug, Deserialize, Clone, Serialize)]
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
