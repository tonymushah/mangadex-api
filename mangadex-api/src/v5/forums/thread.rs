use crate::HttpClientRef;
pub mod post;

use post::CreateForumThreadBuilder;

#[derive(Debug)]
pub struct ForumsThreadsEndpoint{
    http_client : HttpClientRef
}

impl ForumsThreadsEndpoint{
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self{
        Self { http_client }
    }
    pub fn post(&self) -> CreateForumThreadBuilder{
        CreateForumThreadBuilder::default().http_client(self.http_client.clone())
    }
}