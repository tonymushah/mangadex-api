use mangadex_api_types::TagGroup;
use serde::{Deserialize, Serialize};

use crate::v5::{localizedstring_array_or_map, LocalizedString};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TagAttributes {
    pub name: LocalizedString,
    #[serde(with = "localizedstring_array_or_map")]
    pub description: LocalizedString,
    pub group: TagGroup,
    pub version: u32,
}
