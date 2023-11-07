pub mod get;

use mangadex_api_types::ReportCategory;

use crate::HttpClientRef;
use get::ListReasonsBuilder;

/// Report endpoint handler builder.
#[derive(Clone, Debug)]
pub struct CategoryEndpoint {
    http_client: HttpClientRef,
    category: ReportCategory,
}

impl CategoryEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, category: ReportCategory) -> Self {
        Self {
            http_client,
            category,
        }
    }
    pub fn get(&self) -> ListReasonsBuilder {
        ListReasonsBuilder::default()
            .category(self.category)
            .http_client(self.http_client.clone())
    }
}
