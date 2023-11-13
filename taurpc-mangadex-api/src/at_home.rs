pub mod server;

use mangadex_api_schema::{v5::AtHomeServer, Limited};
use server::AtHomeServerParams;
use tauri::{Runtime, Window};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::at_home::server::id::get::GetAtHomeServerBuilder, MangaDexClient};

use crate::Result;

#[taurpc::procedures(path = "mangadex.at_home")]
pub trait AtHome {
    async fn server<R: Runtime>(
        params: AtHomeServerParams,
        window: Window<R>,
    ) -> Result<Limited<AtHomeServer>>;
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl AtHome for MangaDexClient {
    async fn server<R: Runtime>(
        self,
        params: AtHomeServerParams,
        _window: Window<R>,
    ) -> Result<Limited<AtHomeServer>> {
        let builder: GetAtHomeServerBuilder = params.into();
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
}
