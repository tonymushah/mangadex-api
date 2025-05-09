use std::{
    cell::{BorrowError, BorrowMutError},
    fmt::Display,
};

use derive_builder::UninitializedFieldError;
use schema::MangaDexErrorResponse_ as MangaDexErrorResponse;

use crate::RelationshipType;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum RelationshipConversionError {
    #[error("The input relationship type {input} is incompatible with {inner}")]
    InvalidInputRelationshipType {
        input: RelationshipType,
        inner: RelationshipType,
    },
    #[error("The {0} related attributes is not found")]
    AttributesNotFound(RelationshipType),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error when parsing a URL.
    ///
    /// This should not happen.
    #[error("error parsing the URL")]
    ParseUrlError(#[from] url::ParseError),

    #[error("there was an error from the MangaDex servers (HTTP {0}): {1}")]
    ServerError(u16, String),

    #[error("failed to send a request to MangaDex: {0:?}")]
    RequestError(#[from] reqwest::Error),

    #[error("a field is missing when building the request: {0:?}")]
    UninitializedFieldError(#[from] UninitializedFieldError),

    /// Error when building the request.
    #[error("failed to build the request: {0:?}")]
    BuilderError(#[from] BuilderError),

    #[error("missing auth tokens; please log in to MangaDex")]
    MissingTokens,

    #[error("missing client info; please insert the client_id and client_secret")]
    MissingClientInfo,

    #[error("missing captcha; please insert it or solve a captcha")]
    MissingCaptcha,

    #[error("not a valid username: {0}")]
    UsernameError(String),

    #[error("not a valid password: {0}")]
    PasswordError(String),

    #[error("an error occurred while pinging the MangaDex server")]
    PingError,

    /// Errors returned from the MangaDex API request.
    #[error("an error occurred with the MangaDex API request: {0:?}")]
    Api(#[from] MangaDexErrorResponse),

    /// Error while building the request struct.
    #[error("failed to build a request: {0}")]
    RequestBuilderError(String),

    /// Error while parsing the type.
    #[error("an error occurred while parsing the type: {0}")]
    ParseError(String),

    #[error("an error occurred when borrowing the http client")]
    BorrowError(#[from] BorrowError),

    #[error("an error occured when borrowing the http client as mutable")]
    BorrowMutError(#[from] BorrowMutError),

    #[error("an eccor captured")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RateLimitParseError(#[from] crate::rate_limit::RateLimitParseError),

    #[error("Rate Limit Excedeed")]
    RateLimitExcedeed,

    #[error(transparent)]
    ForumThreadTypeParseError(#[from] crate::forum_thread::ForumThreadTypeParseError),

    #[error(transparent)]
    RelationshipConversionError(#[from] RelationshipConversionError),

    #[error("This file {0} was skipped")]
    SkippedDownload(String),
    #[error("The {0} variant should only be `0` or `1`")]
    IncludeEnumsParsing(String),

    #[error("{0}")]
    UnknowSource(String),
}

impl Error {
    pub fn unknow<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        Self::UnknowSource(source.into())
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Error::ParseUrlError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::ServerError(port, host) => serializer.serialize_str(
                format!("there was an error from the MangaDex servers (HTTP {host}): {port}")
                    .as_str(),
            ),
            Error::RequestError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::BuilderError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::MissingTokens => {
                serializer.serialize_str("missing auth tokens; please log in to MangaDex")
            }
            Error::UsernameError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::PasswordError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::PingError => serializer.serialize_str(
                "Cannot ping the Mangadex API. Please checkout your internet connection",
            ),
            Error::Api(e) => e.serialize(serializer),
            Error::RequestBuilderError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::ParseError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::BorrowError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::BorrowMutError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::Io(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::UninitializedFieldError(e) => serializer.serialize_str(
                format!("the field {} must be initialized", e.field_name()).as_str(),
            ),
            Error::RateLimitParseError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::RateLimitExcedeed => serializer.serialize_str("Rate Limit Excedeed"),
            Error::ForumThreadTypeParseError(e) => serializer.serialize_str(e.to_string().as_str()),
            Error::MissingClientInfo => serializer.serialize_str(
                "missing client info; please insert the client_id and client_secret",
            ),
            Error::MissingCaptcha => {
                serializer.serialize_str("missing captcha; please insert it or solve a captcha")
            }
            Error::SkippedDownload(e) => {
                serializer.serialize_str(format!("This file {} was skipped", e).as_str())
            }
            Error::RelationshipConversionError(e) => {
                serializer.serialize_str(format!("This file {} was skipped", e).as_str())
            }
            Error::IncludeEnumsParsing(e) => serializer
                .serialize_str(format!("The {e} variant should only be `0` or `1`").as_str()),
            Error::UnknowSource(e) => serializer.serialize_str(e),
        }
    }
}

#[cfg(feature = "specta")]
impl specta::Type for Error {
    fn inline(
        _opts: specta::DefOpts,
        _generics: &[specta::DataType],
    ) -> std::result::Result<specta::DataType, specta::ExportError> {
        Ok(specta::DataType::Primitive(specta::PrimitiveType::String))
    }
}

pub mod schema {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::ResultType;

    #[derive(Debug, thiserror::Error, Deserialize, Serialize)]
    #[error("Bad request")]
    #[cfg_attr(feature = "specta", derive(specta::Type))]
    pub struct MangaDexErrorResponse_ {
        #[serde(default = "ResultType::error")]
        pub result: ResultType,
        #[serde(default)]
        pub errors: Vec<MangaDexError>,
    }

    #[derive(Debug, thiserror::Error, PartialEq, Eq, Deserialize, Clone, Serialize)]
    #[error("API error")]
    #[cfg_attr(feature = "specta", derive(specta::Type))]
    pub struct MangaDexError {
        pub id: Uuid,
        /// HTTP status code.
        pub status: u16,
        /// Error title.
        pub title: Option<String>,
        /// Description about the error.
        pub detail: Option<String>,
        /// Provides insight into why the request failed.
        ///
        /// # Captcha Errors (400)
        ///
        /// The error may have been caused by one of the following:
        ///
        /// - Captcha challenge result was wrong.
        /// - The Captcha Verification service was down.
        /// - Other, refer to the error message and the `errorCode` value.
        ///
        /// # Rate Limit, Captcha Required (403)
        ///
        /// Some endpoints may require captchas to proceed, in order to slow down automated malicious
        /// traffic. Legitimate users might also be affected, based on the frequency of write requests
        /// or due certain endpoints being particularly sensitive to malicious use, such as user signup.
        ///
        /// Once an endpoint decides that a captcha needs to be solved,
        /// a 403 Forbidden response will be returned, with the error code `captcha_required_exception`.
        /// The sitekey needed for recaptcha to function is provided in both the
        /// `X-Captcha-Sitekey` header field, as well as in the error context,
        /// specified as `siteKey` parameter.
        ///
        /// The captcha result of the client can either be passed into the repeated original request
        /// with the `X-Captcha-Result` header or alternatively to the `POST /captcha/solve` endpoint.
        /// The time a solved captcha is remembered varies across different endpoints and can also be
        /// influenced by individual client behavior.
        ///
        /// Authentication is not required for the `POST /captcha/solve` endpoint, captchas are tracked
        /// both by client ip and logged in user id. If you are logged in, you want to send the session
        /// token along, so you validate the captcha for your client ip and user id at the same time,
        /// but it is not required.
        // TODO: Use enum representations once the structure of this field is known.
        // See: https://serde.rs/enum-representations.html
        pub context: Option<HashMap<String, String>>,
    }
}

#[derive(Debug)]
pub enum BuilderError {
    /// Uninitialized field
    UninitializedField(String),
    /// Custom validation error
    ValidationError(String),
}

impl From<String> for BuilderError {
    fn from(value: String) -> Self {
        Self::ValidationError(value)
    }
}

impl From<UninitializedFieldError> for BuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self::UninitializedField(value.field_name().to_string())
    }
}
impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::UninitializedField(s) => {
                f.write_str(format!("the field {s} must initialized").as_str())
            }
            BuilderError::ValidationError(s) => f.write_str(s.as_str()),
        }
    }
}
impl std::error::Error for BuilderError {}
