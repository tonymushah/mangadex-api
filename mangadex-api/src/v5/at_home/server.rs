pub mod get;

use get::GetAtHomeServerBuilder;

use crate::HttpClientRef;

#[derive(Clone, Debug)]
pub struct ServerEndPoint {
    http_client: HttpClientRef,
}

impl ServerEndPoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> GetAtHomeServerBuilder {
        GetAtHomeServerBuilder::default().http_client(self.http_client.clone())
    }
}
