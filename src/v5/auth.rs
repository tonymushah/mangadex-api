//! Authentication endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Auth>

mod check_token;
mod login;
mod logout;
mod refresh_token;

use crate::v5::auth::check_token::CheckTokenBuilder;
use crate::v5::auth::login::LoginBuilder;
use crate::v5::auth::logout::LogoutBuilder;
use crate::v5::auth::refresh_token::RefreshTokenBuilder;
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
    pub fn login(&self) -> LoginBuilder {
        LoginBuilder::default().http_client(self.http_client.clone())
    }

    /// Log out of an account.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication/operation/post-auth-logout>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn logout(&self) -> LogoutBuilder {
        LogoutBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a new session token from the refresh token.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication/operation/post-auth-refresh>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn refresh_token(&self) -> RefreshTokenBuilder {
        RefreshTokenBuilder::default().http_client(self.http_client.clone())
    }

    /// Check the current session token and get basic info about the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Auth/get-auth-check>
    pub fn check_token(&self) -> CheckTokenBuilder {
        CheckTokenBuilder::default().http_client(self.http_client.clone())
    }
}
