#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::author::post::CreateAuthorBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::AuthorData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_schema::v5::LocalizedString;
use url::Url;

#[derive(serde::Serialize, serde::Deserialize, specta::Type, Debug, Clone)]
pub struct AuthorCreateParams {
    pub name: String,

    #[serde(default)]
    pub biography: Option<LocalizedString>,

    #[serde(default)]
    pub twitter: Option<Url>,

    #[serde(default)]
    pub pixiv: Option<Url>,

    #[serde(default)]
    pub melon_book: Option<Url>,

    #[serde(default)]
    pub fan_box: Option<Url>,

    #[serde(default)]
    pub booth: Option<Url>,

    #[serde(default)]
    pub nico_video: Option<Url>,

    #[serde(default)]
    pub skeb: Option<Url>,

    #[serde(default)]
    pub fantia: Option<Url>,

    #[serde(default)]
    pub tumblr: Option<Url>,

    #[serde(default)]
    pub youtube: Option<Url>,

    #[serde(default)]
    pub weibo: Option<Url>,

    #[serde(default)]
    pub naver: Option<Url>,

    #[serde(default)]
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
