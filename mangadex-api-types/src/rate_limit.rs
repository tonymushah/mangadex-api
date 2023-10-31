use std::num::ParseIntError;

use reqwest::{
    header::{HeaderMap, HeaderName, ToStrError},
    Response,
};
use serde::Serialize;
use time::OffsetDateTime;

use crate::MangaDexDateTime;

#[derive(Serialize, Debug)]
pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
    pub retry_after: MangaDexDateTime,
}

impl RateLimit {
    fn get_limit_header() -> HeaderName {
        HeaderName::from_static("X-RateLimit-Limit")
    }
    fn get_remaining_header() -> HeaderName {
        HeaderName::from_static("X-RateLimit-Remaining")
    }
    fn get_retry_after_header() -> HeaderName {
        HeaderName::from_static("X-RateLimit-Retry-After")
    }
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
            .get(Self::get_limit_header())
            .ok_or(RateLimitParseError::HeaderNotFound(
                Self::get_limit_header().to_string(),
            ))?
            .to_str()?
            .parse()?;
        let remaining: u32 = value
            .get(Self::get_remaining_header())
            .ok_or(RateLimitParseError::HeaderNotFound(
                Self::get_remaining_header().to_string(),
            ))?
            .to_str()?
            .parse()?;
        let retry_after = MangaDexDateTime::from(OffsetDateTime::from_unix_timestamp(
            value
                .get(Self::get_retry_after_header())
                .ok_or(RateLimitParseError::HeaderNotFound(
                    Self::get_retry_after_header().to_string(),
                ))?
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
