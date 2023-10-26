//! Scanlation group endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/ScanlationGroup>

pub mod get;
pub mod post;
pub mod id;

use get::ListGroupBuilder;
use post::CreateGroupBuilder;
use id::IdEndpoint;
use uuid::Uuid;

use crate::HttpClientRef;

/// Scanlation group endpoint handler builder.
#[derive(Debug)]
pub struct ScanlationGroupBuilder {
    http_client: HttpClientRef,
}

impl ScanlationGroupBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> ListGroupBuilder{
        ListGroupBuilder::default().http_client(self.http_client.clone())
    } 
    pub fn post(&self) -> CreateGroupBuilder{
        CreateGroupBuilder::default().http_client(self.http_client.clone())
    }
    pub fn id(&self, id: Uuid) -> IdEndpoint{
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
