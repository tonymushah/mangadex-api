use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::{format_description, OffsetDateTime};

pub(crate) const MANGADEX_DATETIME_FORMAT: &str = "[year]-[month]-[day]T[hour]:[minute]:[second]";

pub(crate) const MANGADEX_DATETIME_FORMAT_DESER: &str =
    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]";

/// Newtype struct for handling datetime fields in MangaDex.
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaDexDateTime(OffsetDateTime);

impl MangaDexDateTime {
    pub fn new(datetime: &OffsetDateTime) -> Self {
        Self(*datetime)
    }
}

impl From<OffsetDateTime> for MangaDexDateTime {
    fn from(datetime: OffsetDateTime) -> Self {
        Self(datetime)
    }
}

impl AsRef<OffsetDateTime> for MangaDexDateTime {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Serialize for MangaDexDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description::parse(MANGADEX_DATETIME_FORMAT).unwrap();

        serializer.serialize_str(&self.as_ref().format(&format).unwrap())
    }
}

impl<'de> Deserialize<'de> for MangaDexDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let format = format_description::parse(MANGADEX_DATETIME_FORMAT_DESER).unwrap();

        let datetime = OffsetDateTime::parse(&s, &format).unwrap();

        Ok(MangaDexDateTime(datetime))
    }
}

impl std::fmt::Display for MangaDexDateTime {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let format = format_description::parse(MANGADEX_DATETIME_FORMAT).unwrap();

        fmt.write_str(&self.as_ref().format(&format).unwrap())
    }
}

impl Default for MangaDexDateTime {
    fn default() -> Self {
        MangaDexDateTime(OffsetDateTime::now_utc())
    }
}
