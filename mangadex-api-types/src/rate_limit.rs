use std::num::ParseIntError;

use reqwest::{
    header::{HeaderMap,ToStrError},
    Response,
};
use serde::Serialize;
use time::OffsetDateTime;

use crate::MangaDexDateTime;

pub const LIMIT: &str = "x-ratelimit-limit";

pub const REMAINING: &str = "x-ratelimit-remaining";

pub const RETRY_AFTER: &str = "x-ratelimit-retry-after";

#[derive(Serialize, Debug)]
pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
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
        println!("parsing len {}", value.len());
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
