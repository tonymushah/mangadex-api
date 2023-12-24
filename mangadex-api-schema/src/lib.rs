//! MangaDex API response object types.

mod bind;
pub mod v5;
use std::borrow::Cow;
use std::ops::Deref;

use mangadex_api_types::error::schema::MangaDexErrorResponse_ as MangaDexErrorResponse;
use mangadex_api_types::error::Error;
use mangadex_api_types::rate_limit::RateLimit;
use mangadex_api_types::{RelationshipType, ResponseType, ResultType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::v5::Relationship;

pub trait UrlSerdeQS {
    fn query_qs<T: Serialize>(self, query: &T) -> Self;
}

impl UrlSerdeQS for url::Url {
    fn query_qs<T: Serialize>(mut self, query: &T) -> Self {
        self.set_query(Some(
            &serde_qs::to_string(query).expect("failed to encode query string"),
        ));
        self
    }
}

pub trait FromResponse: Sized {
    type Response;

    fn from_response(res: Self::Response) -> Self;
}

pub trait Endpoint {
    type Query: Serialize;
    type Body: Serialize;
    type Response: FromResponse;

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn path(&self) -> Cow<str>;

    fn require_auth(&self) -> bool {
        false
    }

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }

    fn multipart(&self) -> Option<reqwest::multipart::Form> {
        None
    }
}

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

impl<T> FromResponse for ApiData<T>
where
    T: DeserializeOwned,
{
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
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

impl<A> FromResponse for ApiObject<A> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
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

impl<A> FromResponse for ApiObjectNoRelationships<A> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
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
pub struct NoData {
    #[serde(default)]
    result: ResultType,
}

impl FromResponse for NoData {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl<T> FromResponse for Result<T, Error> {
    type Response = ApiResult<T, MangaDexErrorResponse>;

    fn from_response(value: Self::Response) -> Self {
        value.into_result().map_err(|e| e.into())
    }
}

impl<T> FromResponse for Vec<Result<T, Error>> {
    type Response = Vec<ApiResult<T, MangaDexErrorResponse>>;

    fn from_response(value: Self::Response) -> Self {
        value
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect()
    }
}

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// This struct is used for rate limited endpoint
/// `rate_limit` is for the rate limit metadata
/// `body` is the response data

#[cfg(feature = "serialize")]
#[derive(Debug, Serialize, Clone)]
pub struct Limited<T>
where
    T: Serialize + Clone,
{
    pub rate_limit: RateLimit,
    pub body: T,
}

#[cfg(not(feature = "serialize"))]
#[derive(Debug, Clone)]
pub struct Limited<T>
where
    T: Clone,
{
    pub rate_limit: RateLimit,
    pub body: T,
}

#[cfg(not(feature = "serialize"))]
impl<T> Deref for Limited<T>
where
    T: Clone,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

#[cfg(feature = "serialize")]
impl<T> Deref for Limited<T>
where
    T: Clone + serde::Serialize,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
