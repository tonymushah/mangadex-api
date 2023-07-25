use mangadex_api_types::MangaDexDateTime;
use serde::Deserialize;
use url::Url;

use crate::v5::{localizedstring_array_or_map, LocalizedString};

/// General author information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    #[serde(with = "localizedstring_array_or_map")]
    pub biography: LocalizedString,
    /// <https://twitter.com>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub twitter: Option<Url>,
    /// <https://www.pixiv.net>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub pixiv: Option<Url>,
    /// <https://www.melonbooks.co.jp>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub melon_book: Option<Url>,
    /// <https://www.fanbox.cc>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub fan_box: Option<Url>,
    /// <https://booth.pm>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub booth: Option<Url>,
    /// <https://www.nicovideo.jp>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub nico_video: Option<Url>,
    /// <https://skeb.jp>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub skeb: Option<Url>,
    /// <https://fantia.jp>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub fantia: Option<Url>,
    /// <https://www.tumblr.com>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub tumblr: Option<Url>,
    /// <https://www.youtube.com>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub youtube: Option<Url>,
    /// [https://weibo.cn/u/](https://weibo.cn)
    /// or
    /// [https://m.weibo.cn/u/](https://m.weibo.cn)
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub weibo: Option<Url>,
    /// <https://blog.naver.com/>
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub naver: Option<Url>,
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub website: Option<Url>,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub updated_at: Option<MangaDexDateTime>,
}
