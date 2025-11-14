use serde::{Deserialize, Serialize};

/// The Api Client profile
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum ApiClientProfile {
    #[default]
    Personal,
    Public,
}

impl ApiClientProfile {
    pub fn is_personal(&self) -> bool {
        *self == ApiClientProfile::Personal
    }
    pub fn is_public(&self) -> bool {
        *self == ApiClientProfile::Public
    }
}
