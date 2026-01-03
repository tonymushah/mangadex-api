use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum ResultType {
    #[default]
    Ok,
    Error,
    Ko,
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
