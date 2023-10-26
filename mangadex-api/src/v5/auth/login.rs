use crate::HttpClientRef;

pub mod post;
use post::LoginBuilder;

#[derive(Debug, Clone)]
pub struct LoginEndpoint {
    http_client: HttpClientRef,
}

impl LoginEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn post(&self) -> LoginBuilder {
        LoginBuilder::default().http_client(self.http_client.clone())
    }
}
