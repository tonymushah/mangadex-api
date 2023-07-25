//! Report endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Report>

#[cfg(not(feature = "deserializable-endpoint"))]
mod create;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod list_reports_by_user;

#[cfg(feature = "deserializable-endpoint")]
pub mod create;
#[cfg(feature = "deserializable-endpoint")]
pub mod list;
#[cfg(feature = "deserializable-endpoint")]
pub mod list_reports_by_user;

use crate::v5::report::create::CreateReportBuilder;
use crate::v5::report::list::ListReasonsBuilder;
use crate::v5::report::list_reports_by_user::ListReportsByUserBuilder;
use crate::HttpClientRef;

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

    /// Get a list of report reasons.
    ///
    /// <https://api.mangadex.org/swagger.html#/Report/get-report-reasons-by-category>
    pub fn list(&self) -> ListReasonsBuilder {
        ListReasonsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a list of reports by the user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Report/get-reports>
    pub fn list_reports_by_user(&self) -> ListReportsByUserBuilder {
        ListReportsByUserBuilder::default().http_client(self.http_client.clone())
    }

    /// Create a new report.
    ///
    /// <https://api.mangadex.org/swagger.html#/Report/post-report>
    pub fn create(&self) -> CreateReportBuilder {
        CreateReportBuilder::default().http_client(self.http_client.clone())
    }
}
