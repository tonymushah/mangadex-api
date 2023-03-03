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
    /// <https://api.mangadex.org/swagger.html#/Account/post-account-create>
    pub fn create(&self) -> CreateAccountBuilder {
        CreateAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Activate a MangaDex account after creating one.
    ///
    /// <https://api.mangadex.org/swagger.html#/Account/get-account-activate-code>
    pub fn activate(&self) -> ActivateAccountBuilder {
        ActivateAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Resend the account activation code.
    ///
    /// <https://api.mangadex.org/swagger.html#/Account/post-account-activate-resend>
    pub fn resend_activation_code(&self) -> ResendActivationCodeBuilder {
        ResendActivationCodeBuilder::default().http_client(self.http_client.clone())
    }

    /// Initiate the account recovery process.
    ///
    /// <https://api.mangadex.org/swagger.html#/Account/post-account-recover>
    pub fn recover(&self) -> RecoverAccountBuilder {
        RecoverAccountBuilder::default().http_client(self.http_client.clone())
    }

    /// Complete the account recovery process.
    ///
    /// <https://api.mangadex.org/swagger.html#/Account/post-account-recover-code>
    pub fn complete_recovery(&self) -> CompleteAccountRecoveryBuilder {
        CompleteAccountRecoveryBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if an account username is available.
    ///
    /// <https://api.mangadex.org/swagger.html#/Account/get-account-available>
    pub fn check_username_available(&self) -> CheckUsernameAvailableBuilder {
        CheckUsernameAvailableBuilder::default().http_client(self.http_client.clone())
    }
}
