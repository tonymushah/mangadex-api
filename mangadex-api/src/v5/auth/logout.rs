pub mod post;

use post::LogoutBuilder;

use crate::HttpClientRef;

#[derive(Clone, Debug)]
pub struct LogoutEndpoint {
    http_client: HttpClientRef,
}

impl LogoutEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn post(&self) -> LogoutBuilder {
        LogoutBuilder::default().http_client(self.http_client.clone())
    }
}
