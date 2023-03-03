use mangadex_api_types::{MangaDexDateTime, ReportStatus};
use serde::Deserialize;

/// User submitted report information.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserReportAttributes {
    pub details: String,
    pub object_id: String,
    pub status: ReportStatus,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
}
