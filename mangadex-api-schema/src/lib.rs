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
#[non_exhaustive]
enum ApiResultDef<T, E> {
    // this migth change in a near future
    /// The server might often use `ko` in some 404 responses.
    #[serde(rename = "ok", alias = "ko")]
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
#[non_exhaustive]
pub struct ApiData<T> {
    #[serde(default)]
    pub result: ResultType,
    pub response: ResponseType,
    pub data: T,
}

impl<T> ApiData<T> {
    fn new(data: T) -> ApiData<T> {
        Self {
            response: ResponseType::Entity,
            data,
            result: ResultType::Ok,
        }
    }
}

impl<T> Default for ApiData<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct ApiObject<A> {
    pub id: Uuid,
    pub type_: RelationshipType,
    pub attributes: A,
    pub relationships: Vec<Relationship>,
}

impl Default for ApiObject<()> {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            type_: RelationshipType::Unknown,
            attributes: (),
            relationships: Vec::new(),
        }
    }
}

pub trait TypedAttributes {
    const TYPE_: RelationshipType;
}

impl<A> Default for ApiObject<A>
where
    A: TypedAttributes + Default,
{
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            type_: A::TYPE_,
            attributes: A::default(),
            relationships: Vec::new(),
        }
    }
}

impl<A> ApiObject<A> {
    pub fn new(id: Uuid, type_: RelationshipType, attr: A) -> Self {
        Self {
            id,
            type_,
            attributes: attr,
            relationships: Default::default(),
        }
    }
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct ApiObjectNoRelationships<A> {
    pub id: Uuid,
    pub type_: RelationshipType,
    pub attributes: A,
}

impl<A> Default for ApiObjectNoRelationships<A>
where
    A: TypedAttributes + Default,
{
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            type_: A::TYPE_,
            attributes: A::default(),
        }
    }
}

impl<A> ApiObjectNoRelationships<A> {
    pub fn new(attributes: A) -> Self {
        Self {
            id: Uuid::nil(),
            type_: RelationshipType::Unknown,
            attributes,
        }
    }
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
#[non_exhaustive]
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

/// There might be some edge cases where some endpoints returns that returns [`ApiObject::id`] that is not an [`Uuid`].
///
/// Currently, only the `GET /manga/{id}/recommendation` have that one.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct ApiObjectStringId<A> {
    pub id: String,
    pub type_: RelationshipType,
    pub attributes: A,
    pub relationships: Vec<Relationship>,
}

impl Default for ApiObjectStringId<()> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            type_: RelationshipType::Unknown,
            attributes: (),
            relationships: Vec::new(),
        }
    }
}

impl<A> Default for ApiObjectStringId<A>
where
    A: TypedAttributes + Default,
{
    fn default() -> Self {
        Self {
            id: Default::default(),
            type_: A::TYPE_,
            attributes: A::default(),
            relationships: Vec::new(),
        }
    }
}

impl<A> ApiObjectStringId<A> {
    pub fn new(id: String, type_: RelationshipType, attr: A) -> Self {
        Self {
            id,
            type_,
            attributes: attr,
            relationships: Default::default(),
        }
    }
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

impl<T> TryFrom<ApiObjectStringId<T>> for ApiObject<T> {
    type Error = uuid::Error;
    fn try_from(value: ApiObjectStringId<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.parse()?,
            type_: value.type_,
            attributes: value.attributes,
            relationships: value.relationships,
        })
    }
}

impl<T> From<ApiObject<T>> for ApiObjectStringId<T> {
    fn from(value: ApiObject<T>) -> Self {
        Self {
            id: value.id.to_string(),
            type_: value.type_,
            attributes: value.attributes,
            relationships: value.relationships,
        }
    }
}

/// An utility trait for finding relationships quickly from an object by their [RelationshipType].
pub trait RelationedObject {
    fn find_relationships(&self, type_: RelationshipType) -> Vec<&Relationship>;
    fn find_first_relationships(&self, _type_: RelationshipType) -> Option<&Relationship> {
        None
    }
}

impl<T> RelationedObject for Vec<T>
where
    T: RelationedObject,
{
    fn find_first_relationships(&self, type_: RelationshipType) -> Option<&Relationship> {
        self.iter().find_map(|d| d.find_first_relationships(type_))
    }
    fn find_relationships(&self, type_: RelationshipType) -> Vec<&Relationship> {
        self.iter()
            .flat_map(|d| d.find_relationships(type_))
            .collect()
    }
}

impl<T> RelationedObject for v5::Results<T>
where
    T: RelationedObject,
{
    fn find_relationships(&self, type_: RelationshipType) -> Vec<&Relationship> {
        self.data.find_relationships(type_)
    }
    fn find_first_relationships(&self, type_: RelationshipType) -> Option<&Relationship> {
        self.data.find_first_relationships(type_)
    }
}

macro_rules! impl_api_obj {
    ($type:ty) => {
        impl<T> RelationedObject for $type {
            fn find_relationships(&self, type_: RelationshipType) -> Vec<&Relationship> {
                self.relationships
                    .iter()
                    .filter(|rel| rel.type_ == type_)
                    .collect()
            }
            fn find_first_relationships(&self, type_: RelationshipType) -> Option<&Relationship> {
                self.relationships.iter().find(|rel| rel.type_ == type_)
            }
        }
    };
}

impl_api_obj!(ApiObject<T>);
impl_api_obj!(ApiObjectStringId<T>);
