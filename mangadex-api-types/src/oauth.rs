use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum GrantTypeSupported {
    RefreshToken,
    Password,
    AuthorizationCode,
    ClientCredentials,
}

impl ToString for GrantTypeSupported {
    fn to_string(&self) -> String {
        match self {
            GrantTypeSupported::RefreshToken => "refresh_token".to_string(),
            GrantTypeSupported::Password => "password".to_string(),
            GrantTypeSupported::AuthorizationCode => "authorization_code".to_string(),
            GrantTypeSupported::ClientCredentials => "client_credentials".to_string(),
        }
    }
}
