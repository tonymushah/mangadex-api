use mangadex_api_types::{MangaDexDateTime, ReportStatus};
use serde::{Deserialize, Serialize};

/// User submitted report information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserReportAttributes {
    pub details: String,
    pub object_id: String,
    pub status: ReportStatus,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
}
