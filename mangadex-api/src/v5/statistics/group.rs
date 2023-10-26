use crate::HttpClientRef;

use self::{get::FindGroupStatisticsBuilder, id::IdEndpoint};

pub mod get;
pub mod id;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct GroupEndpoint {
    http_client: HttpClientRef,
}

impl GroupEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn get(&self) -> FindGroupStatisticsBuilder {
        FindGroupStatisticsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn id(&self, id: uuid::Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}