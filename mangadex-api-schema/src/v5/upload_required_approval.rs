use mangadex_api_types::ResultType;
use serde::Deserialize;

use crate::FromResponse;

/// User Settings response.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[allow(unused)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadRequiredApproval {
    #[serde(default)]
    pub result: ResultType,
    pub requires_approval: bool,
}

impl From<bool> for UploadRequiredApproval {
    fn from(value: bool) -> Self {
        Self {
            result: Default::default(),
            requires_approval: value,
        }
    }
}

impl From<UploadRequiredApproval> for bool {
    fn from(value: UploadRequiredApproval) -> Self {
        value.requires_approval
    }
}

impl FromResponse for UploadRequiredApproval {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}
