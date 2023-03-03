use serde::{Deserialize, Serialize};

/// Upload file source.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadSource {
    Local,
    Remote,
}
