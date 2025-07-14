#[macro_export]
macro_rules! include_enums {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
        #[cfg_attr(feature = "specta", derive(specta::Type))]
        #[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
        #[serde(try_from = "u8", into = "u8")]
        #[non_exhaustive]
        pub enum $name {
            Include,
            Exclude,
        }

        impl TryFrom<u8> for $name {
            type Error = Error;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::Exclude),
                    1 => Ok(Self::Include),
                    _ => Err(Error::IncludeEnumsParsing(String::from(stringify!($name)))),
                }
            }
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                match value {
                    $name::Exclude => 0,
                    $name::Include => 1,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(match self {
                    Self::Include => "Include",
                    Self::Exclude => "Exclude",
                })
            }
        }
    };
}
