use mangadex_api_types::TagGroup;
use serde::Deserialize;

use crate::v5::{localizedstring_array_or_map, LocalizedString};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TagAttributes {
    pub name: LocalizedString,
    #[serde(with = "localizedstring_array_or_map")]
    pub description: LocalizedString,
    pub group: TagGroup,
    pub version: u32,
}
