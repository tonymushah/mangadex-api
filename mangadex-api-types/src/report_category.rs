use serde::{Deserialize, Serialize};

/// Report reasons for submitting reports to the MangaDex staff.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum ReportCategory {
    Author,
    Chapter,
    Manga,
    ScanlationGroup,
    User,
}

impl std::fmt::Display for ReportCategory {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Author => "author",
            Self::Chapter => "chapter",
            Self::Manga => "manga",
            Self::ScanlationGroup => "scanlation_group",
            Self::User => "user",
        })
    }
}
