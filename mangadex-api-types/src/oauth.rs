use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum GrantTypeSupported {
    RefreshToken,
    Password,
    AuthorizationCode,
    ClientCredentials,
}

impl Display for GrantTypeSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GrantTypeSupported::RefreshToken => "refresh_token",
            GrantTypeSupported::Password => "password",
            GrantTypeSupported::AuthorizationCode => "authorization_code",
            GrantTypeSupported::ClientCredentials => "client_credentials",
        })
    }
}
