use serde::{Deserialize};

use crate::v5::AuthTokens;
use crate::FromResponse;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LoginResponse {
    pub token: AuthTokens,
}

impl FromResponse for LoginResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}
