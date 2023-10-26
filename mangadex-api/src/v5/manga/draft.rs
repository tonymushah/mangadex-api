use crate::HttpClientRef;

pub mod get;
pub mod id;

use get::ListMangaDraftsBuilder;
use id::IdEndpoint;
use uuid::Uuid;

#[derive(Debug)]
pub struct DraftEndpoint{
    http_client: HttpClientRef
}

impl DraftEndpoint{
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self{
        Self{ http_client }
    } 
    pub fn get(&self) -> ListMangaDraftsBuilder{
        ListMangaDraftsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn id(&self, id: Uuid) -> IdEndpoint{
        IdEndpoint::new(self.http_client.clone(), id)
    }
}