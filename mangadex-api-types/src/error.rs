pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("The {0} variant should only be `0` or `1`")]
    IncludeEnumsParsing(String),
    #[error("not a valid username: {0}")]
    UsernameError(String),

    #[error("not a valid password: {0}")]
    PasswordError(String),

    /// Error while parsing the type.
    #[error("an error occurred while parsing the type: {0}")]
    ParseError(String),
}
