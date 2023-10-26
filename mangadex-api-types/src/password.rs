use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

const MIN_LEN: usize = 8;
const MAX_LEN: usize = 1024;

/// Type to ensure passwords sent to the API are valid.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Password(String);

impl Password {
    /// Validate and instantiate a new `Password`.
    pub fn parse<T: Into<String>>(password: T) -> Result<Self> {
        let password = password.into();

        // The length checks should check grapheme count instead of raw character count.
        let is_too_short = password.len() < MIN_LEN;

        let is_too_long = password.len() > MAX_LEN;

        if is_too_short || is_too_long {
            Err(Error::PasswordError(format!(
                "The password must be between {} and {} characters",
                MIN_LEN, MAX_LEN
            )))
        } else {
            Ok(Self(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_fewer_than_8_char_parses_error() {
        let short_password = Password::parse("a".repeat(MIN_LEN - 1));

        assert!(short_password.is_err());
    }

    #[test]
    fn password_more_than_1024_char_parses_error() {
        let long_password = Password::parse("a".repeat(MAX_LEN + 1));

        assert!(long_password.is_err());
    }
}
