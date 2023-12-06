use serde::{Deserialize, Serialize};

/// Report reasons for submitting reports to the MangaDex staff.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum ReportStatus {
    Accepted,
    Autoresolved,
    Refused,
    Waiting,
}

impl std::fmt::Display for ReportStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Accepted => "accepted",
            Self::Autoresolved => "autoresolved",
            Self::Refused => "refused",
            Self::Waiting => "waiting",
        })
    }
}
