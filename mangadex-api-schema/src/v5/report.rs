use mangadex_api_types::ReportCategory;
use serde::{Deserialize};

use crate::v5::LocalizedString;

/// Report reason response object.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct ReportReasonAttributes {
    pub reason: LocalizedString,
    pub details_required: bool,
    pub category: ReportCategory,
    pub version: u32,
}
