//! Authentication endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Auth>

pub mod check;
#[cfg(feature = "legacy-auth")]
pub mod login;
#[cfg(feature = "legacy-auth")]
pub mod logout;
#[cfg(feature = "legacy-auth")]
pub mod refresh;

use crate::v5::auth::check::CheckEndpoint;
#[cfg(feature = "legacy-auth")]
use crate::v5::auth::login::LoginEndpoint;
#[cfg(feature = "legacy-auth")]
use crate::v5::auth::logout::LogoutEndpoint;
#[cfg(feature = "legacy-auth")]
use crate::v5::auth::refresh::RefreshEndpoint;
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

    /// Log into an account.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication/operation/post-auth-login>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-auth")]
    pub fn login(&self) -> LoginEndpoint {
        LoginEndpoint::new(self.http_client.clone())
    }

    /// Log out of an account.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication/operation/post-auth-logout>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-auth")]
    pub fn logout(&self) -> LogoutEndpoint {
        LogoutEndpoint::new(self.http_client.clone())
    }

    /// Get a new session token from the refresh token.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication/operation/post-auth-refresh>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-auth")]
    pub fn refresh(&self) -> RefreshEndpoint {
        RefreshEndpoint::new(self.http_client.clone())
    }

    /// Check the current session token and get basic info about the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Auth/get-auth-check>
    pub fn check(&self) -> CheckEndpoint {
        CheckEndpoint::new(self.http_client.clone())
    }
}
