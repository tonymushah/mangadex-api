//! General CustomList information.

use mangadex_api_types::CustomListVisibility;
use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: CustomListVisibility,
    pub version: u32,
}
