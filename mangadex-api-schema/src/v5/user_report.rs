use mangadex_api_types::{MangaDexDateTime, ReportStatus};
use serde::{Deserialize};

/// User submitted report information.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserReportAttributes {
    pub details: String,
    pub object_id: String,
    pub status: ReportStatus,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
}
