use uuid::Uuid;

use crate::HttpClientRef;

use self::get::FindChapterStatisticsBuilder;
use self::id::IdEndpoint;

pub mod get;
pub mod id;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct ChapterEndpoint {
    http_client: HttpClientRef,
}

impl ChapterEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn get(&self) -> FindChapterStatisticsBuilder {
        FindChapterStatisticsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
