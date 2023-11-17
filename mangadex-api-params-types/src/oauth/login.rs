#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::oauth::login::RetriveTokensBuilder;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::{Password, Username};

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct OAuthLoginParams {
    pub username: String,
    pub password: String,
}

#[cfg(feature = "mangadex-api-resolver")]
impl TryFrom<OAuthLoginParams> for RetriveTokensBuilder {
    type Error = mangadex_api_types::error::Error;
    fn try_from(value: OAuthLoginParams) -> Result<Self, Self::Error> {
        let password = Password::parse(value.password)?;
        let username = Username::parse(value.username)?;
        let mut builder = RetriveTokensBuilder::default();
        builder.password(password);
        builder.username(username);
        Ok(builder)
    }
}
