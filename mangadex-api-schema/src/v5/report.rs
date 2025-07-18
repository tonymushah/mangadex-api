use mangadex_api_types::{RelationshipType, ReportCategory};
use serde::Deserialize;

use crate::{TypedAttributes, v5::LocalizedString};

/// Report reason response object.
#[derive(Clone, Debug, Deserialize, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct ReportReasonAttributes {
    pub reason: LocalizedString,
    pub details_required: bool,
    pub category: Option<ReportCategory>,
    pub version: u32,
}

impl TypedAttributes for ReportReasonAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::ReportReason;
}
