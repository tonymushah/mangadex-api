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
