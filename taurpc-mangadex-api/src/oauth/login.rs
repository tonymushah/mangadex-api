#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::oauth::login::RetriveTokensBuilder;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::{Password, Username};

#[taurpc::ipc_type]
pub struct OAuthLoginParams {
    pub username: String,
    pub password: String,
}

#[cfg(feature = "mangadex-api-resolver")]
impl TryFrom<OAuthLoginParams> for RetriveTokensBuilder {
    type Error = crate::Error;
    fn try_from(value: OAuthLoginParams) -> Result<Self, Self::Error> {
        let password = Password::parse(value.password)
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)?;
        let username = Username::parse(value.username)
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)?;
        let mut builder = RetriveTokensBuilder::default();
        builder.password(password);
        builder.username(username);
        Ok(builder)
    }
}
