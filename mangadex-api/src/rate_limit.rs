use std::{num::ParseIntError, ops::Deref};

use reqwest::{
    header::{HeaderMap, ToStrError},
    Response,
};
use serde::Serialize;
use time::OffsetDateTime;

use mangadex_api_types::MangaDexDateTime;

pub const LIMIT: &str = "x-ratelimit-limit";

pub const REMAINING: &str = "x-ratelimit-remaining";

pub const RETRY_AFTER: &str = "x-ratelimit-retry-after";

/// This `RateLimit` struct contains all the data needed for rate limit handling
/// It can be parsed via a [`reqwest::Response`] or [`reqwest::header::HeaderMap`]
/// ```rust
///     use mangadex_api_types_rust::rate_limit::{LIMIT, REMAINING, RETRY_AFTER, RateLimit, RateLimitParseError};
///     use reqwest::header::{HeaderMap, HeaderValue};
///
///     fn main() -> Result<(), RateLimitParseError> {
///         let mut headers = HeaderMap::new();
///         headers.append(RETRY_AFTER, HeaderValue::from_static("1698723860"));
///         headers.append(LIMIT, HeaderValue::from_static("40"));
///         headers.append(REMAINING, HeaderValue::from_static("39"));
///         assert_eq!(headers.len(), 3);
///         let rate_limit: RateLimit = TryFrom::try_from(&headers)?;
///         assert_eq!(rate_limit.limit, 40);
///         assert_eq!(rate_limit.remaining, 39);
///         Ok(())
///     }
/// ```
///
#[derive(Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct RateLimit {
    /// value from `x-ratelimit-limit` header
    pub limit: u32,
    /// value from `x-ratelimit-remaining` header
    pub remaining: u32,
    /// value from `x-ratelimit-retry-after` header
    /// It's normally an [`i64`] [(Unix timestamp)](https://www.unixtimestamp.com/)
    /// but can be parsed as a [`crate::MangaDexDateTime`]
    pub retry_after: MangaDexDateTime,
}

#[derive(Debug, thiserror::Error)]
pub enum RateLimitParseError {
    #[error("this header {0} is not found")]
    HeaderNotFound(String),
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ComponnentRangeError(#[from] time::error::ComponentRange),
}

impl TryFrom<&HeaderMap> for RateLimit {
    type Error = RateLimitParseError;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        let limit: u32 = value
            .iter()
            .find(|(name, _)| (*name).eq(&LIMIT))
            .ok_or(RateLimitParseError::HeaderNotFound(LIMIT.to_string()))?
            .1
            .to_str()?
            .parse()?;
        let remaining: u32 = value
            .iter()
            .find(|(name, _)| (*name).eq(&REMAINING))
            .ok_or(RateLimitParseError::HeaderNotFound(REMAINING.to_string()))?
            .1
            .to_str()?
            .parse()?;
        let retry_after = MangaDexDateTime::from(OffsetDateTime::from_unix_timestamp(
            value
                .iter()
                .find(|(name, _)| (*name).eq(&RETRY_AFTER))
                .ok_or(RateLimitParseError::HeaderNotFound(RETRY_AFTER.to_string()))?
                .1
                .to_str()?
                .parse::<i64>()?,
        )?);
        Ok(Self {
            limit,
            remaining,
            retry_after,
        })
    }
}

impl TryFrom<&Response> for RateLimit {
    type Error = RateLimitParseError;

    fn try_from(value: &Response) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.headers())
    }
}

#[cfg(test)]
mod tests {
    use crate::rate_limit::{LIMIT, REMAINING, RETRY_AFTER};

    use super::{RateLimit, RateLimitParseError};
    use reqwest::header::{HeaderMap, HeaderValue};

    #[test]
    fn test_ratelimit_parsing() -> Result<(), RateLimitParseError> {
        let mut headers = HeaderMap::new();
        headers.append(RETRY_AFTER, HeaderValue::from_static("1698723860"));
        headers.append(LIMIT, HeaderValue::from_static("40"));
        headers.append(REMAINING, HeaderValue::from_static("39"));
        assert_eq!(headers.len(), 3);

        let rate_limit: RateLimit = TryFrom::try_from(&headers)?;
        assert_eq!(rate_limit.limit, 40);
        assert_eq!(rate_limit.remaining, 39);
        Ok(())
    }
}

/// This struct is used for rate limited endpoint
/// `rate_limit` is for the rate limit metadata
/// `body` is the response data
#[derive(Debug, Serialize, Clone)]
pub struct Limited<T> {
    pub rate_limit: RateLimit,
    pub body: T,
}

impl<T> Deref for Limited<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
