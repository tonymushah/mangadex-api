use std::str::FromStr;
use std::string::ParseError;

use serde::{Deserialize, Serialize};

macro_rules! languages {
    (
        $(
            $( #[$meta:meta] )*
            $lang:ident => $code:literal,
        )*
    ) => {
        /// Languages supported by MangaDex.
        #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
        #[non_exhaustive]
        pub enum Language {
            $(
                $( #[$meta] )*
                #[serde(rename = $code)]
                $lang,
            )*
            #[serde(rename = "NULL")]
            Unknown,
        }

        impl Language {
            /// Get the ISO 639-1 2-letter code representation.
            pub fn code2(&self) -> &str {
                match self {
                    $(
                        Self::$lang => $code,
                    )*
                    Self::Unknown => "NULL",
                }
            }
        }

        impl From<&str> for Language {
            /// Parse a `Language` type from a string.
            ///
            /// This function's value parameter is case-insensitive.
            fn from(value: &str) -> Self {
                match value.to_lowercase().as_str() {
                    $(
                        $code => Self::$lang,
                    )*
                    _ => Self::Unknown,
                }
            }
        }

        impl FromStr for Language {
            type Err = ParseError;

            /// Parse a `Language` type from a string.
            ///
            /// This function's value parameter is case-insensitive.
            fn from_str(value: &str) -> Result<Self, ParseError> {
                Ok(
                    match value.to_lowercase().as_str() {
                        $(
                            $code => Self::$lang,
                        )*
                        _ => Self::Unknown,
                    }
                )
            }
        }
    };
}

languages! {
    Arabic => "ar",
    Azerbaijani => "az",
    Bengali => "bn",
    Bulgarian => "bg",
    Burmese => "my",
    Catalan => "ca",
    ChineseRomanized => "zh-ro",
    ChineseSimplified => "zh",
    ChineseTraditional => "zh-hk",
    Croatian => "hr",
    Czech => "cs",
    Danish => "da",
    Dutch => "nl",
    English => "en",
    Esperanto => "eo",
    Filipino => "tl",
    Finnish => "fi",
    French => "fr",
    German => "de",
    Greek => "el",
    Hebrew => "he",
    Hindi => "hi",
    Hungarian => "hu",
    Indonesian => "id",
    Italian => "it",
    Japanese => "ja",
    JapaneseRomanized => "ja-ro",
    Kazakh => "kk",
    Korean => "ko",
    KoreanRomanized => "ko-ro",
    Latin => "la",
    Lithuanian => "lt",
    Malay => "ms",
    Mongolian => "mn",
    Nepali => "ne",
    NiloSaharan => "kr",
    Norwegian => "no",
    Persian => "fa",
    Polish => "pl",
    PortugueseBrazilian => "pt-br",
    PortuguesePortugal => "pt",
    Romansh => "rm",
    Romanian => "ro",
    Russian => "ru",
    SerboCroatian => "sr",
    Slovak => "sk",
    SpanishCastilian => "es",
    SpanishLatinAmerican => "es-la",
    Swedish => "sv",
    Tamil => "ta",
    Thai => "th",
    Turkish => "tr",
    Ukrainian => "uk",
    Vietnamese => "vi",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_produces_english_from_en() {
        let lang = Language::from("en");
        assert_eq!(lang, Language::English);
    }

    #[test]
    fn string_produces_japanese_from_capitalized_ja() {
        let lang = Language::from("JA");
        assert_eq!(lang, Language::Japanese);
    }

    #[test]
    fn string_produces_unknown_from_unknown_string() {
        let test_cases = ["foo", "bar", "baz"];
        for test in test_cases {
            let lang = Language::from(test);
            assert_eq!(lang, Language::Unknown);
        }
    }
}
