use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, Eq, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum CustomListVisibility {
    Public,
    #[default]
    Private,
}

impl From<String> for CustomListVisibility {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Public,
        }
    }
}

impl std::fmt::Display for CustomListVisibility {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Public => "public",
            Self::Private => "private",
        })
    }
}
