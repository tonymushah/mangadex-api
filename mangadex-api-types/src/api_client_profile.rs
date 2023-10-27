use serde::{Deserialize, Serialize};

/// The Api Client profile
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ApiClientProfile {
    Personal,
    Public,
}
