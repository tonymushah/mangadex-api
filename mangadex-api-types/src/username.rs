use serde::Serialize;

use crate::error::{Error, Result};

const MIN_LEN: usize = 1;
const MAX_LEN: usize = 64;

/// Type to ensure usernames sent to the API are valid.
#[derive(Debug, Serialize, Clone)]
pub struct Username(String);

impl Username {
    /// Validate and instantiate a new `Username`.
    pub fn parse<T: Into<String>>(username: T) -> Result<Self> {
        let username = username.into();

        // The length checks should check grapheme count instead of raw character count.
        let is_too_short = username.len() < MIN_LEN;

        let is_too_long = username.len() > MAX_LEN;

        if is_too_short || is_too_long {
            Err(Error::UsernameError(format!(
                "The username must be between {} and {} characters",
                MIN_LEN, MAX_LEN
            )))
        } else {
            Ok(Self(username))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_fewer_than_1_char_parses_error() {
        let short_username = Username::parse(&"a".repeat(MIN_LEN - 1));

        assert!(short_username.is_err());
    }

    #[test]
    fn password_more_than_64_char_parses_error() {
        let long_username = Username::parse(&"a".repeat(MAX_LEN + 1));

        assert!(long_username.is_err());
    }
}
