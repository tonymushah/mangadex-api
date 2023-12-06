use std::path::Path;

pub mod api_client;
pub mod at_home;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod cover;
pub mod custom_list;
pub mod feed;
pub mod follows;
pub mod forums;
pub mod legacy;
pub mod manga;
pub mod oauth;
pub mod rating;
pub mod read_marker;
pub mod report;
pub mod scanlation_group;
pub mod settings;
pub mod statistics;
pub mod upload;
pub mod user;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PathBuf(std::path::PathBuf);

impl From<std::path::PathBuf> for PathBuf {
    fn from(value: std::path::PathBuf) -> Self {
        Self(value)
    }
}

impl From<PathBuf> for std::path::PathBuf {
    fn from(value: PathBuf) -> Self {
        value.0
    }
}
impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(PathBuf);
