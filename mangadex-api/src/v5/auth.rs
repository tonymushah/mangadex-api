//! Authentication endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Auth>

pub mod check;

use crate::v5::auth::check::CheckEndpoint;
use crate::HttpClientRef;

/// Authentication endpoint handler builder.
#[derive(Debug)]
pub struct AuthBuilder {
    http_client: HttpClientRef,
}

impl AuthBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Check the current session token and get basic info about the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Auth/get-auth-check>
    pub fn check(&self) -> CheckEndpoint {
        CheckEndpoint::new(self.http_client.clone())
    }
}
