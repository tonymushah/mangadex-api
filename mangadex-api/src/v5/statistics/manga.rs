use crate::HttpClientRef;

use self::{get::FindMangaStatisticsBuilder, id::IdEndpoint};

pub mod get;
pub mod id;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct MangaEndpoint {
    http_client: HttpClientRef,
}

impl MangaEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> FindMangaStatisticsBuilder {
        FindMangaStatisticsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn id(&self, id: uuid::Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
