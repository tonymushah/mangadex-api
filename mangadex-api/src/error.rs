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
