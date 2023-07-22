use mangadex_api_types::TagGroup;
use serde::Deserialize;

use crate::v5::{localizedstring_array_or_map, LocalizedString};

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TagAttributes {
    pub name: LocalizedString,
    #[serde(with = "localizedstring_array_or_map")]
    pub description: LocalizedString,
    pub group: TagGroup,
    pub version: u32,
}
