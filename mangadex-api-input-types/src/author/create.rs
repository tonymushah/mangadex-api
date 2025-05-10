#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::author::post::CreateAuthorBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::AuthorData, Limited};

use mangadex_api_schema::v5::LocalizedString;
use url::Url;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct AuthorCreateParams {
    pub name: String,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub biography: Option<LocalizedString>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub twitter: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub pixiv: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub melon_book: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub fan_box: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub booth: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub nico_video: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub skeb: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub fantia: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub tumblr: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub youtube: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub weibo: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub naver: Option<Url>,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub website: Option<Url>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AuthorCreateParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<AuthorData>> {
        let builder: CreateAuthorBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AuthorCreateParams> for CreateAuthorBuilder {
    fn from(value: AuthorCreateParams) -> Self {
        let mut builder = Self::default();
        builder.name(value.name);
        if let Some(biography) = value.biography {
            builder.biography(biography);
        }
        builder.twitter(value.twitter);
        builder.pixiv(value.pixiv);
        builder.melon_book(value.melon_book);
        builder.fan_box(value.fan_box);
        builder.booth(value.booth);
        builder.nico_video(value.nico_video);
        builder.skeb(value.skeb);
        builder.fantia(value.fantia);
        builder.tumblr(value.tumblr);
        builder.youtube(value.youtube);
        builder.weibo(value.weibo);
        builder.naver(value.naver);
        builder.website(value.website);
        builder
    }
}
