use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentRating {
    Erotica,
    Pornographic,
    Safe,
    Suggestive,
}

impl From<String> for ContentRating {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "erotica" => Self::Erotica,
            "pornographic" => Self::Pornographic,
            "safe" => Self::Safe,
            "suggestive" => Self::Suggestive,
            _ => Self::Safe,
        }
    }
}

impl std::fmt::Display for ContentRating {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Erotica => "Erotica",
            Self::Pornographic => "Pornographic",
            Self::Safe => "Safe",
            Self::Suggestive => "Suggestive",
        })
    }
}
