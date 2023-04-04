use mangadex_api_types::MangaDexDateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::v5::{localizedstring_array_or_map, LocalizedString};

/// General author information.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    #[serde(with = "localizedstring_array_or_map")]
    pub biography: LocalizedString,
    /// <https://twitter.com>
    pub twitter: Option<Url>,
    /// <https://www.pixiv.net>
    pub pixiv: Option<Url>,
    /// <https://www.melonbooks.co.jp>
    pub melon_book: Option<Url>,
    /// <https://www.fanbox.cc>
    pub fan_box: Option<Url>,
    /// <https://booth.pm>
    pub booth: Option<Url>,
    /// <https://www.nicovideo.jp>
    pub nico_video: Option<Url>,
    /// <https://skeb.jp>
    pub skeb: Option<Url>,
    /// <https://fantia.jp>
    pub fantia: Option<Url>,
    /// <https://www.tumblr.com>
    pub tumblr: Option<Url>,
    /// <https://www.youtube.com>
    pub youtube: Option<Url>,
    /// [https://weibo.cn/u/](https://weibo.cn)
    /// or
    /// [https://m.weibo.cn/u/](https://m.weibo.cn)
    pub weibo: Option<Url>,
    /// <https://blog.naver.com/>
    pub naver: Option<Url>,
    pub website: Option<Url>,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub updated_at: Option<MangaDexDateTime>,
}
