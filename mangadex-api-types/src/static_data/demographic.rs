use serde::{Deserialize, Serialize};

/// Target demographic for manga.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, Eq, Default)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum Demographic {
    Shounen,
    Shoujo,
    Seinen,
    Josei,
    #[default]
    None,
}

impl From<String> for Demographic {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "shounen" => Self::Shounen,
            "shoujo" => Self::Shoujo,
            "josei" => Self::Josei,
            "seinen" => Self::Seinen,
            _ => Self::None,
        }
    }
}

impl std::fmt::Display for Demographic {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Shounen => "shounen",
            Self::Shoujo => "shoujo",
            Self::Seinen => "seinen",
            Self::Josei => "josei",
            Self::None => "none",
        };
        fmt.write_str(name)
    }
}
