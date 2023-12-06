use std::str::FromStr;
use std::string::ParseError;

use serde::{Deserialize, Serialize};

/// Mapping types to get the new UUIDs from the legacy, numerical, IDs.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum LegacyMappingType {
    Chapter,
    Group,
    Manga,
    Tag,
}

impl From<&str> for LegacyMappingType {
    /// Parse a `LegacyMappingType` type from a string.
    ///
    /// This function's value parameter is case-insensitive.
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "chapter" => Self::Chapter,
            "group" => Self::Group,
            "manga" => Self::Manga,
            "tag" => Self::Tag,
            _ => Self::Manga,
        }
    }
}

impl FromStr for LegacyMappingType {
    type Err = ParseError;

    /// Parse a `Language` type from a string.
    ///
    /// This function's value parameter is case-insensitive.
    fn from_str(value: &str) -> Result<Self, ParseError> {
        Ok(Self::from(value))
    }
}
