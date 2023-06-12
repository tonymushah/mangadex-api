//! Account endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Account>

mod activate;
mod check_username_available;
mod complete_recovery;
mod create;
mod recover;
mod resend_activation_code;

use crate::v5::account::activate::ActivateAccountBuilder;
use crate::v5::account::check_username_available::CheckUsernameAvailableBuilder;
use crate::v5::account::complete_recovery::CompleteAccountRecoveryBuilder;
use crate::v5::account::create::CreateAccountBuilder;
use crate::v5::account::recover::RecoverAccountBuilder;
use crate::v5::account::resend_activation_code::ResendActivationCodeBuilder;
use crate::HttpClientRef;

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

    /// Create a new MangaDex account.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/post-account-create>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn create(&self) -> CreateAccountBuilder {
        CreateAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Activate a MangaDex account after creating one.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/get-account-activate-code>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn activate(&self) -> ActivateAccountBuilder {
        ActivateAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Resend the account activation code.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/post-account-activate-resend>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn resend_activation_code(&self) -> ResendActivationCodeBuilder {
        ResendActivationCodeBuilder::default().http_client(self.http_client.clone())
    }

    /// Initiate the account recovery process.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/post-account-recover>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn recover(&self) -> RecoverAccountBuilder {
        RecoverAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Complete the account recovery process.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/post-account-recover-code>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn complete_recovery(&self) -> CompleteAccountRecoveryBuilder {
        CompleteAccountRecoveryBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if an account username is available.
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/get-account-available>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    pub fn check_username_available(&self) -> CheckUsernameAvailableBuilder {
        CheckUsernameAvailableBuilder::default().http_client(self.http_client.clone())
    }
}
