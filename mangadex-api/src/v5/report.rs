//! Report endpoint handler.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Report>

pub mod get;
pub mod post;
pub mod reasons;

use crate::HttpClientRef;
use get::ListReportsByUserBuilder;
use post::CreateReportBuilder;
use reasons::ReasonsEndpoint;

/// Report endpoint handler builder.
#[derive(Clone, Debug)]
pub struct ReportBuilder {
    http_client: HttpClientRef,
}

impl ReportBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn get(&self) -> ListReportsByUserBuilder {
        ListReportsByUserBuilder::default().http_client(self.http_client.clone())
    }
    pub fn post(&self) -> CreateReportBuilder {
        CreateReportBuilder::default().http_client(self.http_client.clone())
    }
    pub fn reasons(&self) -> ReasonsEndpoint {
        ReasonsEndpoint::new(self.http_client.clone())
    }
}
