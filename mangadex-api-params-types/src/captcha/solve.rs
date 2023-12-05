#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::captcha::solve::post::SolveCaptchaBuilder, MangaDexClient};
use mangadex_api_schema::Limited;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CaptchaSolveParams {
    pub captcha_challenge: String,
}

#[cfg(feature = "mangadex-api-resolver")]
impl CaptchaSolveParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<()>> {
        let builder: SolveCaptchaBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
            .map(|d| Limited {
                rate_limit: d.rate_limit,
                body: (),
            })
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CaptchaSolveParams> for SolveCaptchaBuilder {
    fn from(value: CaptchaSolveParams) -> Self {
        let mut builder = Self::default();
        builder.captcha_challenge(value.captcha_challenge);
        builder
    }
}
