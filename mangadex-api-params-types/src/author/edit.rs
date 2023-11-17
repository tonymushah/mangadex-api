#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::author::id::put::UpdateAuthorBuilder;

use mangadex_api_schema::v5::LocalizedString;
use url::Url;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct AuthorEditParams {
    pub id: Uuid,

    pub name: Option<String>,

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

    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AuthorEditParams> for UpdateAuthorBuilder {
    fn from(value: AuthorEditParams) -> Self {
        let mut builder = Self::default();
        builder.author_id(value.id);
        if let Some(name) = value.name {
            builder.name(name);
        }
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
        builder.version(value.version);
        builder
    }
}
