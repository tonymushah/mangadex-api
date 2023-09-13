use uuid::Uuid;

use crate::HttpClientRef;

pub mod post;

#[derive(Debug, Clone)]
pub struct DefaultEndpoint{
    http_client: HttpClientRef,
    id: Uuid
}

impl DefaultEndpoint{
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self{
        Self{http_client, id}
    }
    pub fn post(&self){
        todo!("implement the post method please")
    }
}