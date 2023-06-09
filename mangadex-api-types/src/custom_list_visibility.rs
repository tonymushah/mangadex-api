use serde::{Deserialize, Serialize};

/// Visibility setting for CustomLists.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CustomListVisibility {
    Public,
    Private,
}
