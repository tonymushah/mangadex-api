use serde::{Deserialize, Serialize};

/// Determines the behavior of tag interaction when including or excluding tags in the results.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TagSearchMode {
    And,
    Or,
}

impl std::fmt::Display for TagSearchMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::And => "AND",
            Self::Or => "OR",
        })
    }
}
