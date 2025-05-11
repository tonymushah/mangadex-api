use mangadex_api_types::{MangaDexDateTime, ReportStatus};
use serde::Deserialize;

/// User submitted report information.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserReportAttributes {
    pub details: String,
    pub object_id: String,
    pub status: ReportStatus,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub created_at: MangaDexDateTime,
}
