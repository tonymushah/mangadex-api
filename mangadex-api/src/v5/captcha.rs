//! CAPTCHA endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Captcha>

#[cfg(not(feature = "deserializable-endpoint"))]
mod solve;

#[cfg(feature = "deserializable-endpoint")]
pub mod solve;

use crate::v5::captcha::solve::SolveCaptchaBuilder;
use crate::HttpClientRef;

/// CAPTCHA endpoint handler builder.
#[derive(Debug)]
pub struct CaptchaBuilder {
    http_client: HttpClientRef,
}

impl CaptchaBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Solve a CAPTCHA challenge.
    ///
    /// <https://api.mangadex.org/swagger.html#/Captcha/post-captcha-solve>
    pub fn solve(&self) -> SolveCaptchaBuilder {
        SolveCaptchaBuilder::default().http_client(self.http_client.clone())
    }
}
