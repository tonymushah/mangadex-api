use crate::HttpClientRef;

pub mod post;

use post::RefreshTokenBuilder;

#[derive(Clone, Debug)]
pub struct RefreshEndpoint{
    http_client : HttpClientRef
}

impl RefreshEndpoint{
    #[doc(hidden)]
    pub fn new(http_client : HttpClientRef) -> Self{
        Self { http_client }
    }
    pub fn post(&self) -> RefreshTokenBuilder {
        RefreshTokenBuilder::default().http_client(self.http_client.clone())
    }
}