//! MangaDex API response object types.

pub mod v5;
mod bind;
use std::borrow::Cow;

use mangadex_api_types::error::schema::MangaDexErrorResponse;
use mangadex_api_types::error::Error;
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
    pub result : ResultType,
    pub response: ResponseType,
    pub data: T,
}

#[derive(Debug, Default, Deserialize, Clone, )]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiObject<A, T = RelationshipType> {
    pub id: Uuid,
    pub type_: T,
    pub attributes: A,
    pub relationships: Vec<Relationship>,
}

impl<A, T> FromResponse for ApiObject<A, T> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiObjectNoRelationships<A, T = RelationshipType> {
    pub id: Uuid,
    pub type_: T,
    pub attributes: A,
}

impl<A, T> FromResponse for ApiObjectNoRelationships<A, T> {
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
pub struct NoData;

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
