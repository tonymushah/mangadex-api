use crate::HttpClientRef;

use self::post::CompleteAccountRecoveryBuilder;

pub mod post;

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct CodeEndpoint {
    http_client: HttpClientRef,
    code : String
}

impl CodeEndpoint{
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, code : String) -> Self {
        Self { http_client, code }
    }

    pub fn post(&self) -> CompleteAccountRecoveryBuilder{
        CompleteAccountRecoveryBuilder::default().http_client(self.http_client.clone()).code(self.code.clone()).clone()
    }
}