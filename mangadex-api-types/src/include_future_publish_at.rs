use serde::{Deserialize, Serialize};

/// Flag to include future updates in the results.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum IncludeFuturePublishAt {
    Include = 0,
    Exclude = 1,
}

impl std::fmt::Display for IncludeFuturePublishAt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Include => "Include",
            Self::Exclude => "Exclude",
        })
    }
}
