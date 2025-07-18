use serde::{Deserialize, Serialize};

/// Upload file source.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum UploadSource {
    #[default]
    Local,
    Remote,
}
