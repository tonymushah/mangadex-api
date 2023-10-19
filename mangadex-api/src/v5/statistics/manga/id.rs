use crate::HttpClientRef;

use self::get::GetMangaStatisticsBuilder;
use uuid::Uuid;
pub mod get;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid
}

impl IdEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaStatisticsBuilder{
        GetMangaStatisticsBuilder::default().http_client(self.http_client.clone()).manga_id(self.id)
    }
}