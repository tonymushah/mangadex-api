use crate::HttpClientRef;

use self::post::CreateAccountBuilder;

pub mod post;

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct CreateEndpoint {
    http_client: HttpClientRef,
}

impl CreateEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn post(&self) -> CreateAccountBuilder {
        CreateAccountBuilder::default().http_client(self.http_client.clone())
    }
}
