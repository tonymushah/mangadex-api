use crate::HttpClientRef;

use self::get::CheckUsernameAvailableBuilder;

pub mod get;

/// Account endpoint handler builder.
#[derive(Debug)]
pub struct AvailableEndpoint {
    http_client: HttpClientRef,
}

impl AvailableEndpoint{
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn get(&self) -> CheckUsernameAvailableBuilder{
        CheckUsernameAvailableBuilder::default().http_client(self.http_client.clone())
    }
}