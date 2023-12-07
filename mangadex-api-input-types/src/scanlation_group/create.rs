#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::scanlation_group::post::CreateGroupBuilder, MangaDexClient};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::GroupData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_types::MangaDexDuration;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CreateScalantionGroupParam {
    pub name: String,
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
    #[serde(default)]
    pub inactive: Option<bool>,
    /// Nullable.
    #[serde(default)]
    pub publish_delay: Option<MangaDexDuration>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateScalantionGroupParam> for CreateGroupBuilder {
    fn from(value: CreateScalantionGroupParam) -> Self {
        let mut builder = Self::default();
        builder.name(value.name);
        if let Some(website) = value.website {
            builder.website(website);
        }
        if let Some(irc_server) = value.irc_server {
            builder.irc_server(irc_server);
        }
        if let Some(irc_channel) = value.irc_channel {
            builder.irc_channel(irc_channel);
        }
        if let Some(discord) = value.discord {
            builder.discord(discord);
        }
        if let Some(contact_email) = value.contact_email {
            builder.contact_email(contact_email);
        }
        if let Some(description) = value.description {
            builder.description(description);
        }
        if let Some(twitter) = value.twitter {
            builder.twitter(twitter);
        }
        if let Some(manga_updates) = value.manga_updates {
            builder.manga_updates(manga_updates);
        }
        if let Some(inactive) = value.inactive {
            builder.inactive(inactive);
        }
        if let Some(publish_delay) = value.publish_delay {
            builder.publish_delay(publish_delay);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateScalantionGroupParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<GroupData>> {
        <CreateGroupBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
