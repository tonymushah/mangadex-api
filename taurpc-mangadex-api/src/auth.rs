#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::CheckTokenResponse;
use tauri::{Runtime, Window};

use crate::Result;

#[taurpc::procedures(path = "mangadex.auth")]
pub trait Auth {
    async fn check<R: Runtime>(window: Window<R>) -> Result<CheckTokenResponse>;
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl Auth for MangaDexClient {
    async fn check<R: Runtime>(self, _window: Window<R>) -> Result<CheckTokenResponse> {
        self.auth()
            .check()
            .get()
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
}
