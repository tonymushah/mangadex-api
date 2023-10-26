//! Account endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Account>
pub mod activate;
pub mod available;
pub mod create;
pub mod recover;

use crate::HttpClientRef;

use self::{
    activate::ActivateEndpoint, available::AvailableEndpoint, create::CreateEndpoint,
    recover::RecoverEndpoint,
};

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct AccountBuilder {
    http_client: HttpClientRef,
}

impl AccountBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn available(&self) -> AvailableEndpoint {
        AvailableEndpoint::new(self.http_client.clone())
    }

    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn activate(&self) -> ActivateEndpoint {
        ActivateEndpoint::new(self.http_client.clone())
    }

    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn create(&self) -> CreateEndpoint {
        CreateEndpoint::new(self.http_client.clone())
    }

    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn recover(&self) -> RecoverEndpoint {
        RecoverEndpoint::new(self.http_client.clone())
    }
}
