#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::scanlation_group::id::put::UpdateGroupBuilder, MangaDexClient};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::GroupData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_types::{Language, MangaDexDuration};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct EditScanlationGroupParam {
    pub group_id: Uuid,

    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub leader: Option<Uuid>,
    /// Nullable.
    #[serde(default)]
    pub website: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub irc_server: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub irc_channel: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub discord: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub contact_email: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// Nullable.
    #[serde(default)]
    pub twitter: Option<Url>,
    /// Regex: [^https:/\/www\.mangaupdates\.com\/(?:groups|publishers)\.html\?id=\d+](https://www.mangaupdates.com)
    ///
    /// Nullable.
    #[serde(default)]
    pub manga_updates: Option<Url>,
    /// Languages the scanlation primarily translates or uploads works into.
    ///
    /// Nullable.
    #[serde(default)]
    pub focused_languages: Option<Vec<Language>>,
    #[serde(default)]
    pub inactive: Option<bool>,
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    pub publish_delay: Option<MangaDexDuration>,
    /// >= 1
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<EditScanlationGroupParam> for UpdateGroupBuilder {
    fn from(value: EditScanlationGroupParam) -> Self {
        let mut builder = Self::default();
        builder.group_id(value.group_id);
        builder.name(value.name);
        builder.leader(value.leader);
        builder.website(value.website);
        builder.irc_server(value.irc_server);
        builder.irc_channel(value.irc_channel);
        builder.discord(value.discord);
        builder.contact_email(value.contact_email);
        builder.description(value.description);
        builder.twitter(value.twitter);
        builder.manga_updates(value.manga_updates);
        if let Some(focused_languages) = value.focused_languages {
            builder.focused_languages(focused_languages);
        }
        if let Some(inactive) = value.inactive {
            builder.inactive(inactive);
        }
        if let Some(locked) = value.locked {
            builder.locked(locked);
        }
        if let Some(publish_delay) = value.publish_delay {
            builder.publish_delay(publish_delay);
        }
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl EditScanlationGroupParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<GroupData>> {
        <UpdateGroupBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
