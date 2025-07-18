use mangadex_api_types::ResultType;
use serde::Deserialize;

/// User Settings response.
#[derive(Clone, Debug, Deserialize, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[allow(unused)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadRequiredApproval {
    #[serde(default)]
    pub result: ResultType,
    #[serde(default)]
    pub requires_approval: Option<bool>,
}

impl From<bool> for UploadRequiredApproval {
    fn from(value: bool) -> Self {
        Self {
            result: Default::default(),
            requires_approval: Some(value),
        }
    }
}

impl UploadRequiredApproval {
    pub fn is_manga_not_found(&self) -> bool {
        self.requires_approval.is_none()
    }
}

impl From<UploadRequiredApproval> for bool {
    fn from(value: UploadRequiredApproval) -> Self {
        value.requires_approval.unwrap_or_default()
    }
}
