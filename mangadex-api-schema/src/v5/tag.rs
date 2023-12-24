use std::collections::HashMap;

use mangadex_api_types::{Language, RelationshipType, Tag, TagGroup};
use serde::Deserialize;

use crate::{
    v5::{localizedstring_array_or_map, LocalizedString},
    ApiObjectNoRelationships,
};

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

impl From<Tag> for ApiObjectNoRelationships<TagAttributes> {
    fn from(value: Tag) -> Self {
        let mut name: HashMap<Language, String> = HashMap::new();
        name.insert(Language::English, value.to_string());
        Self {
            id: value.into(),
            type_: RelationshipType::Tag,
            attributes: TagAttributes {
                name,
                description: HashMap::default(),
                group: value.into(),
                version: 1,
            },
        }
    }
}
