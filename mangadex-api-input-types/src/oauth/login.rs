#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::oauth::login::RetriveTokensBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::oauth::OAuthTokenResponse;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::{error::Result, Password, Username};

#[derive(serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct OAuthLoginParams {
    pub username: String,
    pub password: String,
}

#[cfg(feature = "mangadex-api-resolver")]
impl OAuthLoginParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<OAuthTokenResponse> {
        let builder: RetriveTokensBuilder = self.try_into()?;
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
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
