use crate::HttpClientRef;

use uuid::Uuid;

pub mod get;

use self::get::GetGroupStatisticsBuilder;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetGroupStatisticsBuilder {
        GetGroupStatisticsBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
}
