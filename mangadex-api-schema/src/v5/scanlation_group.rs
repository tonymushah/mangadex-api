use mangadex_api_types::{Language, MangaDexDateTime, MangaDexDuration, RelationshipType};
use serde::Deserialize;
use url::Url;

use crate::{TypedAttributes, v5::LocalizedString};

/// General scanlation group information.
#[derive(Clone, Debug, Deserialize, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub alt_names: Vec<LocalizedString>,
    pub website: Option<String>,
    pub irc_server: Option<String>,
    pub irc_channel: Option<String>,
    pub discord: Option<String>,
    pub contact_email: Option<String>,
    pub description: Option<String>,
    /// <https://twitter.com>
    ///
    /// Nullable.
    pub twitter: Option<Url>,
    /// Regex: [^https:/\/www\.mangaupdates\.com\/(?:groups|publishers)\.html\?id=\d+](https://www.mangaupdates.com)
    ///
    /// Nullable.
    ///
    pub manga_updates: Option<Url>,
    /// Languages the scanlation primarily translates or uploads works into.
    pub focused_languages: Option<Vec<Language>>,
    pub locked: bool,
    pub official: bool,
    // Known issue: This field is unlisted on the MangaDex documentation but is present in the response.
    pub verified: bool,
    pub inactive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_licensed: Option<bool>,
    /// Should respected ISO 8601 duration specification: <https://en.wikipedia.org/wiki/ISO_8601#Durations>
    ///
    /// Pattern: `^(P([1-9]|[1-9][0-9])D)?(P?([1-9])W)?(P?T(([1-9]|1[0-9]|2[0-4])H)?(([1-9]|[1-5][0-9]|60)M)?(([1-9]|[1-5][0-9]|60)S)?)?$`
    ///
    /// # Examples
    ///
    /// - Two days is `P2D`.
    /// - Two seconds is `PT2S`.
    /// - Six weeks and five minutes is `P6WT5M`.
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub publish_delay: Option<MangaDexDuration>,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub updated_at: MangaDexDateTime,
}

impl TypedAttributes for ScanlationGroupAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::ScanlationGroup;
}
