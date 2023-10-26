use crate::HttpClientRef;

use self::post::RecoverAccountBuilder;

use self::code::CodeEndpoint;

pub mod code;
pub mod post;

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct RecoverEndpoint {
    http_client: HttpClientRef,
}

impl RecoverEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn post(&self) -> RecoverAccountBuilder {
        RecoverAccountBuilder::default().http_client(self.http_client.clone())
    }

    pub fn code(&self, code: String) -> CodeEndpoint {
        CodeEndpoint::new(self.http_client.clone(), code)
    }
}
