#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::captcha::solve::post::SolveCaptchaBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct CaptchaSolveParams {
    pub captcha_challenge: String,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CaptchaSolveParams> for SolveCaptchaBuilder {
    fn from(value: CaptchaSolveParams) -> Self {
        let mut builder = Self::default();
        builder.captcha_challenge(value.captcha_challenge);
        builder
    }
}
