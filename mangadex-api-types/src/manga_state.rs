use serde::{Deserialize, Serialize};

/// Manga state for approval.
///
/// The purpose of these are to discourage troll entries by requiring staff approval.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum MangaState {
    Draft,
    #[default]
    Published,
    Rejected,
    Submitted,
}
