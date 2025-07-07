#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]

//! MangaDex API response object types.

mod bind;
pub mod error;
pub mod v5;

use error::MangaDexErrorResponse_ as MangaDexErrorResponse;
use mangadex_api_types::{RelationshipType, ResponseType, ResultType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::v5::Relationship;

#[derive(Deserialize)]
#[serde(tag = "result", remote = "std::result::Result")]
enum ApiResultDef<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

#[derive(Deserialize)]
#[serde(bound = "T: DeserializeOwned, E: DeserializeOwned")]
pub struct ApiResult<T, E = MangaDexErrorResponse>(
    #[serde(with = "ApiResultDef")] std::result::Result<T, E>,
);

impl<T, E> ApiResult<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        self.0
    }
}

/// API response for a single entity containing an [`ApiObject`] in the `data` field.
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiData<T> {
    #[serde(default)]
    pub result: ResultType,
    pub response: ResponseType,
    pub data: T,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiObject<A> {
    pub id: Uuid,
    pub type_: RelationshipType,
    pub attributes: A,
    pub relationships: Vec<Relationship>,
}

impl<A> ApiObject<A> {
    pub fn find_relationships(&self, type_: RelationshipType) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|rel| rel.type_ == type_)
            .collect()
    }
    pub fn find_first_relationships(&self, type_: RelationshipType) -> Option<&Relationship> {
        self.relationships.iter().find(|rel| rel.type_ == type_)
    }
}

impl<T> From<ApiObject<T>> for ApiObjectNoRelationships<T> {
    fn from(value: ApiObject<T>) -> Self {
        Self {
            id: value.id,
            type_: value.type_,
            attributes: value.attributes,
        }
    }
}

impl<T> ApiObject<T> {
    pub fn drop_relationships(self) -> ApiObjectNoRelationships<T> {
        self.into()
    }
}

impl<T> ApiObjectNoRelationships<T> {
    pub fn with_relathionships(self, rel: Option<Vec<Relationship>>) -> ApiObject<T> {
        let mut res: ApiObject<T> = self.into();
        let mut rels = rel.unwrap_or_default();
        res.relationships.append(&mut rels);
        res
    }
}

impl<T> From<ApiObjectNoRelationships<T>> for ApiObject<T> {
    fn from(value: ApiObjectNoRelationships<T>) -> Self {
        Self {
            id: value.id,
            type_: value.type_,
            attributes: value.attributes,
            relationships: Vec::new(),
        }
    }
}

impl<T> PartialEq for ApiObject<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.type_ == other.type_
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiObjectNoRelationships<A> {
    pub id: Uuid,
    pub type_: RelationshipType,
    pub attributes: A,
}

/// Placeholder to hold response bodies that will be discarded.
///
/// `Result<()>` can't be used with the macro return type because it expects a unit type,
/// so a temporary struct is used.
///
/// # Examples
///
/// ```text
/// endpoint! {
///     POST "/captcha/solve",
///     #[body] SolveCaptcha<'_>,
///     #[discard_result] Result<NoData> // `Result<()>` results in a deserialization error despite discarding the result.
/// }
#[derive(Debug, Default, Deserialize, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NoData {
    #[serde(default)]
    result: ResultType,
}

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
