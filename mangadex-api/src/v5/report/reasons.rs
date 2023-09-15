pub mod category;
use mangadex_api_types::ReportCategory;

use category::CategoryEndpoint;
use crate::HttpClientRef;

/// Report endpoint handler builder.
#[derive(Clone, Debug)]
pub struct ReasonsEndpoint {
    http_client: HttpClientRef,
}

impl ReasonsEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn category(&self, category : ReportCategory) -> CategoryEndpoint {
        CategoryEndpoint::new(self.http_client.clone(), category)
    }
}