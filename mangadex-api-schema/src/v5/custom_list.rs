//! General CustomList information.

use mangadex_api_types::{CustomListVisibility, RelationshipType};
use serde::Deserialize;

use crate::TypedAttributes;

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: CustomListVisibility,
    pub version: u32,
}

impl TypedAttributes for CustomListAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::CustomList;
}
