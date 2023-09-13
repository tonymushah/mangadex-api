use crate::HttpClientRef;

pub mod get;

use get::CheckTokenBuilder;

#[derive(Clone, Debug)]
pub struct CheckEndpoint {
    http_client: HttpClientRef,
}

impl CheckEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> CheckTokenBuilder {
        CheckTokenBuilder::default().http_client(self.http_client.clone())
    }
}
