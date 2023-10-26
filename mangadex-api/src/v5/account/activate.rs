pub mod code;
pub mod resend;

use crate::HttpClientRef;

use self::{code::CodeEndpoint, resend::ResendEndpoint};

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct ActivateEndpoint {
    http_client: HttpClientRef,
}

impl ActivateEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn code(&self, code: String) -> CodeEndpoint {
        CodeEndpoint::new(self.http_client.clone(), code)
    }
    pub fn resend(&self) -> ResendEndpoint {
        ResendEndpoint::new(self.http_client.clone())
    }
}
