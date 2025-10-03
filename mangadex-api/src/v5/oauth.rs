pub mod login;
pub mod refresh_token;

use login::RetriveTokensBuilder;
use refresh_token::RefreshTokensBuilder;

use crate::HttpClientRef;

create_endpoint_node! {
    #[name] OAuthBuider OAuthBuiderMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        login() -> RetriveTokensBuilder;
        refresh() -> RefreshTokensBuilder;
    }
}

impl OAuthBuiderMethods for OAuthBuider {
    fn login(&self) -> RetriveTokensBuilder {
        RetriveTokensBuilder::default().http_client(<&Self as Into<HttpClientRef>>::into(self))
    }
    fn refresh(&self) -> RefreshTokensBuilder {
        RefreshTokensBuilder::default().http_client(<&Self as Into<HttpClientRef>>::into(self))
    }
}

#[derive(Debug, serde::Deserialize)]
struct OAuthError {
    error: String,
}

impl OAuthError {
    async fn handle_resp(res: reqwest::Response) -> crate::error::Error {
        crate::error::Error::OauthError {
            code: res.status().as_u16(),
            reason: res.json::<Self>().await.ok().map(|b| b.error),
        }
    }
}
