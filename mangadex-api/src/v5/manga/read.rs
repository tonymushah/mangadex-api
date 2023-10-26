use std::ops::Deref;

use crate::HttpClientRef;

pub mod get;

use get::GetReadChaptersBuilder;

#[derive(Debug)]
pub struct ReadEndpoint {
    http_client: HttpClientRef,
}

// TODO Add Deref for endpoint that have get(&self)

impl ReadEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> GetReadChaptersBuilder {
        GetReadChaptersBuilder::default().http_client(self.http_client.clone())
    }
}
