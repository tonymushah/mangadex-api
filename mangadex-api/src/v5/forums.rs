use crate::HttpClientRef;
pub mod thread;

use thread::ForumsThreadsEndpoint;
#[derive(Debug)]
pub struct ForumsEndpoint {
    http_client: HttpClientRef,
}

impl ForumsEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn thread(&self) -> ForumsThreadsEndpoint {
        ForumsThreadsEndpoint::new(self.http_client.clone())
    }
}
