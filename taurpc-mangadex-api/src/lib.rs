pub mod api_client;
pub mod at_home;
pub mod auth;
pub mod author;
pub mod manga;
pub mod oauth;

use std::fmt::Display;

pub use api_client::*;
pub use at_home::*;
pub use auth::*;
pub use author::*;
pub use manga::*;
pub use oauth::*;

use mangadex_api_types::ResultType;

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct TauRPCMAngadexAPIError {
    pub result: ResultType,
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
}

pub(crate) type Error = TauRPCMAngadexAPIError;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{} : {}", self.type_, self.message)
    }
}

impl From<mangadex_api_types::error::Error> for Error {
    fn from(value: mangadex_api_types::error::Error) -> Self {
        match value {
            mangadex_api_types::error::Error::ParseUrlError(e) => Self {
                result: ResultType::Error,
                type_: String::from("ParseUrlError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::ServerError(code, e) => Self {
                result: ResultType::Error,
                type_: String::from("ServerError"),
                message: format!("{code} - {e}"),
            },
            mangadex_api_types::error::Error::RequestError(e) => Self {
                result: ResultType::Error,
                type_: String::from("RequestError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::UninitializedFieldError(e) => Self {
                result: ResultType::Error,
                type_: String::from("UninitializedFieldError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::BuilderError(e) => Self {
                result: ResultType::Error,
                type_: String::from("BuilderError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::MissingTokens => Self {
                result: ResultType::Error,
                type_: String::from("MissingTokens"),
                message: String::from("The authentification token is missing"),
            },
            mangadex_api_types::error::Error::MissingClientInfo => Self {
                result: ResultType::Error,
                type_: String::from("MissingClientInfo"),
                message: String::from(r#"The "client_id" or the "client_secret" is missing"#),
            },
            mangadex_api_types::error::Error::MissingCaptcha => Self {
                result: ResultType::Error,
                type_: String::from("MissingCaptcha"),
                message: String::from("The captcha key is missing"),
            },
            mangadex_api_types::error::Error::UsernameError(message) => Self {
                result: ResultType::Error,
                type_: String::from("UsernameError"),
                message,
            },
            mangadex_api_types::error::Error::PasswordError(message) => Self {
                result: ResultType::Error,
                type_: String::from("PasswordError"),
                message,
            },
            mangadex_api_types::error::Error::PingError => Self {
                result: ResultType::Error,
                type_: String::from("PingError"),
                message: String::from("Unable to ping the Mangadex API"),
            },
            mangadex_api_types::error::Error::Api(e) => Self {
                result: ResultType::Error,
                type_: String::from("MangaDexErrorResponse"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::RequestBuilderError(message) => Self {
                result: ResultType::Error,
                type_: String::from("RequestBuilderError"),
                message,
            },
            mangadex_api_types::error::Error::ParseError(message) => Self {
                result: ResultType::Error,
                type_: String::from("ParseError"),
                message,
            },
            mangadex_api_types::error::Error::BorrowError(e) => Self {
                result: ResultType::Error,
                type_: String::from("BorrowError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::BorrowMutError(e) => Self {
                result: ResultType::Error,
                type_: String::from("BorrowMutError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::Io(e) => Self {
                result: ResultType::Error,
                type_: String::from("Io"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::RateLimitParseError(e) => Self {
                result: ResultType::Error,
                type_: String::from("RateLimitParseError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::RateLimitExcedeed => Self {
                result: ResultType::Error,
                type_: String::from("RateLimitExcedeed"),
                message: String::from("Rate Limit Excedeed !"),
            },
            mangadex_api_types::error::Error::ForumThreadTypeParseError(e) => Self {
                result: ResultType::Error,
                type_: String::from("ForumThreadTypeParseError"),
                message: e.to_string(),
            },
            mangadex_api_types::error::Error::SkippedDownload(filename) => Self {
                result: ResultType::Error,
                type_: String::from("SkippedDownload"),
                message: format!("The file '{filename}' was skipped"),
            },
            mangadex_api_types::error::Error::UnexpectedError(e) => Self {
                result: ResultType::Error,
                type_: String::from("UnexpectedError"),
                message: e.to_string(),
            },
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
