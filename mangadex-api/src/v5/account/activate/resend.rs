use crate::HttpClientRef;

use self::post::ResendActivationCodeBuilder;
pub mod post;

#[derive(Debug)]
pub struct ResendEndpoint {
    http_client: HttpClientRef,
}

impl ResendEndpoint{
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn post(&self) -> ResendActivationCodeBuilder{
        ResendActivationCodeBuilder::default().http_client(self.http_client.clone())
    }
}