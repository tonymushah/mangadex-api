pub mod id;

use crate::HttpClientRef;
use uuid::Uuid;

use id::IdEndpoint;

#[derive(Clone, Debug)]
pub struct ServerEndPoint {
    http_client: HttpClientRef,
}

impl ServerEndPoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
