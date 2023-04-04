use serde::{Deserialize, Serialize};

/// Flag to include future updates in the results.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum IncludeExternalUrl {
    Include = 0,
    Exclude = 1,
}

impl std::fmt::Display for IncludeExternalUrl {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Include => "Include",
            Self::Exclude => "Exclude",
        })
    }
}
