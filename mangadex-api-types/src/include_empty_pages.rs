use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Flag to include future updates in the results.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[serde(try_from = "u8", into = "u8")]
pub enum IncludeFuturePages {
    Include,
    Exclude,
}

impl TryFrom<u8> for IncludeFuturePages {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Exclude),
            1 => Ok(Self::Include),
            _ => Err(Error::IncludeFuturePagesParsing),
        }
    }
}

impl From<IncludeFuturePages> for u8 {
    fn from(value: IncludeFuturePages) -> Self {
        match value {
            IncludeFuturePages::Exclude => 0,
            IncludeFuturePages::Include => 1,
        }
    }
}

impl std::fmt::Display for IncludeFuturePages {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Include => "Include",
            Self::Exclude => "Exclude",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Debug, Serialize)]
    struct TestStruct {
        value: IncludeFuturePages,
    }
    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&TestStruct {
                value: IncludeFuturePages::Exclude
            })
            .unwrap(),
            r#"{"value":0}"#
        );
    }
}
