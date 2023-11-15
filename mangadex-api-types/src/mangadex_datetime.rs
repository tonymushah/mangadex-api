use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::{format_description, OffsetDateTime};

pub(crate) const MANGADEX_DATETIME_DE_FORMAT: &str =
    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]";

pub(crate) const MANGADEX_DATETIME_SER_FORMAT: &str =
    "[year]-[month]-[day]T[hour]:[minute]:[second]";

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
        let format = format_description::parse(MANGADEX_DATETIME_SER_FORMAT).unwrap();

        serializer.serialize_str(&self.as_ref().format(&format).unwrap())
    }
}

impl<'de> Deserialize<'de> for MangaDexDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let format_ser = format_description::parse(MANGADEX_DATETIME_SER_FORMAT).unwrap();

        let format_des = format_description::parse(MANGADEX_DATETIME_DE_FORMAT).unwrap();

        let datetime = {
            if let Ok(datetime) = OffsetDateTime::parse(&s, &format_des) {
                datetime
            } else {
                OffsetDateTime::parse(&s, &format_ser).unwrap()
            }
        };

        Ok(MangaDexDateTime(datetime))
    }
}

impl std::fmt::Display for MangaDexDateTime {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let format = format_description::parse(MANGADEX_DATETIME_DE_FORMAT).unwrap();

        fmt.write_str(&self.as_ref().format(&format).unwrap())
    }
}

impl Default for MangaDexDateTime {
    fn default() -> Self {
        MangaDexDateTime(OffsetDateTime::now_utc())
    }
}
