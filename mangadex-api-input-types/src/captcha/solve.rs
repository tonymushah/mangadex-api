#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::rate_limit::Limited;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::captcha::solve::post::SolveCaptchaBuilder, MangaDexClient};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
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
            .map(|d| d.drop_body())
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
