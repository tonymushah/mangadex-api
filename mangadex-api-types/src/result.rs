use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum ResultType {
    Ok,
    Error,
    Ko,
}

impl Default for ResultType {
    fn default() -> Self {
        Self::Ok
    }
}

impl ResultType {
    pub fn ok() -> Self {
        Self::Ok
    }
    pub fn error() -> Self {
        Self::Error
    }
    pub fn ko() -> Self {
        Self::Ko
    }
}
