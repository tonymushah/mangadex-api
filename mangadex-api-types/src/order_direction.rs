use serde::{Deserialize, Serialize};

/// "Order by" directions for manga results.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

// TODO: Remove this.
impl std::fmt::Display for OrderDirection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        })
    }
}
