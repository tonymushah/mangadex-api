use std::fmt::Display;

use derive_builder::UninitializedFieldError;
use mangadex_api_schema::error::RelationshipConversionError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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

    #[error("an error occurred while pinging the MangaDex server")]
    PingError,

    /// Errors returned from the MangaDex API request.
    #[error("an error occurred with the MangaDex API request: {0:?}")]
    Api(#[from] mangadex_api_schema::error::MangaDexErrorResponse_),

    /// Error while building the request struct.
    #[error("failed to build a request: {0}")]
    RequestBuilderError(String),

    /// Error while parsing the type.
    #[error("an error occurred while parsing the type: {0}")]
    ParseError(String),

    #[error("an eccor captured")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RateLimitParseError(#[from] crate::rate_limit::RateLimitParseError),

    #[error("Rate Limit Excedeed")]
    RateLimitExcedeed,

    #[error(transparent)]
    ForumThreadTypeParseError(#[from] mangadex_api_types::forum_thread::ForumThreadTypeParseError),

    #[error(transparent)]
    RelationshipConversionError(#[from] RelationshipConversionError),

    #[error(transparent)]
    Types(#[from] mangadex_api_types::error::Error),

    #[error("This file {0} was skipped")]
    SkippedDownload(String),

    #[error("The API is temporarily anavailable. Reason: {}", if let Some (reason) = .0 {
        &reason
    } else {
        "Unknown"
    })]
    ServiceUnavailable(Option<String>),

    #[cfg(feature = "oauth")]
    #[error("Got Oauth Error response (status: {}, reason: {})", .code, reason.as_ref().map_or("...", |v| v))]
    OauthError { code: u16, reason: Option<String> },

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
        format!("{self}").serialize(serializer)
    }
}

#[derive(Debug)]
#[non_exhaustive]
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
