use mangadex_api_types::RelationshipType;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ForumThreadObject<A, T = RelationshipType> {
    pub type_: T,
    pub id: u32,
    pub attributes: A,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ForumThreadAttributes {
    pub replies_count: u32,
}
