pub mod login;

use login::OAuthLoginParams;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::oauth::login::RetriveTokensBuilder, MangaDexClient};
use mangadex_api_schema::v5::oauth::OAuthTokenResponse;
use tauri::{Runtime, Window};

use crate::Result;

#[taurpc::procedures(path = "mangadex.oauth")]
pub trait OAuth {
    async fn login<R: Runtime>(
        params: OAuthLoginParams,
        window: Window<R>,
    ) -> Result<OAuthTokenResponse>;
    async fn refresh<R: Runtime>(window: Window<R>) -> Result<OAuthTokenResponse>;
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl OAuth for MangaDexClient {
    async fn login<R: Runtime>(
        self,
        params: OAuthLoginParams,
        _window: Window<R>,
    ) -> Result<OAuthTokenResponse> {
        let builder = <RetriveTokensBuilder as TryFrom<OAuthLoginParams>>::try_from(params)?;
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn refresh<R: Runtime>(self, _window: Window<R>) -> Result<OAuthTokenResponse> {
        self.oauth()
            .refresh()
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
}
