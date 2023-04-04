//! General CustomList information.

use mangadex_api_types::CustomListVisibility;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: CustomListVisibility,
    pub version: u32,
}
