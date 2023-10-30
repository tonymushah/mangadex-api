use crate::HttpClientRef;

use uuid::Uuid;

use self::get::GetChapterStatisticsBuilder;

pub mod get;

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
    pub fn get(&self) -> GetChapterStatisticsBuilder {
        GetChapterStatisticsBuilder::default()
            .http_client(self.http_client.clone())
            .chapter_id(self.id)
    }
}
