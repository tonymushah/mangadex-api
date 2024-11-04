use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

const SECONDS_PER_MINUTE: u64 = 60;
const MINUTES_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;
const SECONDS_PER_HOUR: u64 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: u64 = HOURS_PER_DAY * SECONDS_PER_HOUR;
const SECONDS_PER_WEEK: u64 = DAYS_PER_WEEK * SECONDS_PER_DAY;

/// Newtype tuple struct for handling duration fields in MangaDex.
///
/// Should respected ISO 8601 duration specification: <https://en.wikipedia.org/wiki/ISO_8601#Durations>
///
/// Pattern: `^(P([1-9]|[1-9][0-9])D)?(P?([1-9])W)?(P?T(([1-9]|1[0-9]|2[0-4])H)?(([1-9]|[1-5][0-9]|60)M)?(([1-9]|[1-5][0-9]|60)S)?)?$`
///
/// Only the following units are considered to/from the ISO 8601 duration format:
///
/// - Weeks
/// - Days
/// - Hours
/// - Minutes
/// - Seconds
///
/// # Examples
///
/// - Two days is `P2D`.
/// - Two seconds is `PT2S`.
/// - Six weeks and five minutes is `P6WT5M`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Copy)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaDexDuration(Duration);

impl MangaDexDuration {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl AsRef<Duration> for MangaDexDuration {
    fn as_ref(&self) -> &Duration {
        &self.0
    }
}

impl std::fmt::Display for MangaDexDuration {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(format!("{:#?}", self.as_ref()).as_str())
    }
}

impl Serialize for MangaDexDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let output = duration_to_iso_8601(self.as_ref());
        serializer.serialize_str(&output)
    }
}

impl<'de> Deserialize<'de> for MangaDexDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: String = Deserialize::deserialize(deserializer)?;

        let duration = match iso_8601_to_duration(&raw) {
            Ok(d) => Ok(d),
            Err(msg) => Err(serde::de::Error::custom(msg)),
        }?;

        Ok(Self(duration))
    }
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(MangaDexDuration);
/// Parse an ISO 8601 duration string and return a `std::time::Duration` struct.
///
/// Should respected ISO 8601 duration specification: <https://en.wikipedia.org/wiki/ISO_8601#Durations>
///
/// Pattern: `^(P([1-9]|[1-9][0-9])D)?(P?([1-9])W)?(P?T(([1-9]|1[0-9]|2[0-4])H)?(([1-9]|[1-5][0-9]|60)M)?(([1-9]|[1-5][0-9]|60)S)?)?$`
///
/// Only the following units are considered from the ISO 8601 duration format:
///
/// - Weeks
/// - Days
/// - Hours
/// - Minutes
/// - Seconds
///
/// # Examples
///
/// - Two days is `P2D`.
/// - Two seconds is `PT2S`.
/// - Six weeks and five minutes is `P6WT5M`.
// Disclaimer: The method in which this function parses the ISO 8601 duration format is naïve but is functional.
// TODO: Fix this hacky solution.
fn iso_8601_to_duration(date_interval: &str) -> Result<Duration, String> {
    let mut secs: u64 = 0;
    let mut num = "".to_string();
    let mut invalid_input = false;

    let mut it = date_interval.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'P' | 'T' => {
                it.next();
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                num += c.to_string().as_str();
                it.next();
            }
            'W' => {
                secs += num.parse::<u64>().unwrap()
                    * DAYS_PER_WEEK
                    * HOURS_PER_DAY
                    * MINUTES_PER_HOUR
                    * SECONDS_PER_MINUTE;
                num = "".to_string();
                it.next();
            }
            'D' => {
                secs += num.parse::<u64>().unwrap()
                    * HOURS_PER_DAY
                    * MINUTES_PER_HOUR
                    * SECONDS_PER_MINUTE;
                num = "".to_string();
                it.next();
            }
            'H' => {
                secs += num.parse::<u64>().unwrap() * MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
                num = "".to_string();
                it.next();
            }
            'M' => {
                secs += num.parse::<u64>().unwrap() * SECONDS_PER_MINUTE;
                num = "".to_string();
                it.next();
            }
            'S' => {
                secs += num.parse::<u64>().unwrap();
                num = "".to_string();
                it.next();
            }
            _ => {
                invalid_input = true;
                break;
            }
        }
    }

    if invalid_input {
        return Err(format!("invalid DateInterval '{}'", date_interval));
    }

    Ok(Duration::from_secs(secs))
}

/// Convert a `std::time::Duration` struct into a ISO 8601 duration string.
///
/// Should respected ISO 8601 duration specification: <https://en.wikipedia.org/wiki/ISO_8601#Durations>
///
/// Pattern: `^(P([1-9]|[1-9][0-9])D)?(P?([1-9])W)?(P?T(([1-9]|1[0-9]|2[0-4])H)?(([1-9]|[1-5][0-9]|60)M)?(([1-9]|[1-5][0-9]|60)S)?)?$`
///
/// Only the following units are considered to the ISO 8601 duration format:
///
/// - Weeks
/// - Days
/// - Hours
/// - Minutes
/// - Seconds
///
/// # Examples
///
/// - Two days is `P2D`.
/// - Two seconds is `PT2S`.
/// - Six weeks and five minutes is `P6WT5M`.
// The method in which this function serializes the ISO 8601 duration format is naïve but is functional.
// TODO: Fix this hacky solution.
fn duration_to_iso_8601(duration: &Duration) -> String {
    let mut secs = duration.as_secs();

    let weeks = secs / SECONDS_PER_WEEK;
    secs %= SECONDS_PER_WEEK;

    let days = secs / SECONDS_PER_DAY;
    secs %= SECONDS_PER_DAY;

    let hours = secs / SECONDS_PER_HOUR;
    secs %= SECONDS_PER_HOUR;

    let minutes = secs / SECONDS_PER_MINUTE;
    secs %= SECONDS_PER_MINUTE;

    let mut duration_period = "".to_string();
    if weeks > 0 {
        duration_period += &format!("{}W", weeks)
    }
    if days > 0 {
        duration_period += &format!("{}D", days)
    }
    let duration_period = format!("P{}", duration_period);

    let mut time_elements = "".to_string();
    if duration_period == "P" || hours > 0 || minutes > 0 || secs > 0 {
        time_elements += "T";

        if hours > 0 {
            time_elements += &format!("{}H", hours);
        }

        if minutes > 0 {
            time_elements += &format!("{}M", minutes);
        }

        if time_elements == "T" || secs > 0 {
            time_elements += &format!("{}S", secs);
        }
    }

    format!("{}{}", duration_period, time_elements)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn iso_8601_to_duration_works() {
        let test_cases = [
            (
                "P2D",
                Duration::from_secs(2 * HOURS_PER_DAY * MINUTES_PER_HOUR * SECONDS_PER_MINUTE),
            ),
            ("PT2S", Duration::from_secs(2)),
            (
                "P6WT5M",
                Duration::from_secs(
                    (6 * DAYS_PER_WEEK * HOURS_PER_DAY * MINUTES_PER_HOUR * SECONDS_PER_MINUTE)
                        + (5 * SECONDS_PER_MINUTE),
                ),
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(iso_8601_to_duration(input).unwrap(), expected);
        }
    }

    #[test]
    fn duration_to_iso_8601_works() {
        let test_cases = [
            (
                Duration::from_secs(2 * HOURS_PER_DAY * MINUTES_PER_HOUR * SECONDS_PER_MINUTE),
                "P2D",
            ),
            (Duration::from_secs(2), "PT2S"),
            (
                Duration::from_secs(
                    (6 * DAYS_PER_WEEK * HOURS_PER_DAY * MINUTES_PER_HOUR * SECONDS_PER_MINUTE)
                        + (5 * SECONDS_PER_MINUTE),
                ),
                "P6WT5M",
            ),
            (Duration::from_secs(0), "PT0S"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(duration_to_iso_8601(&input), expected);
        }
    }
}
