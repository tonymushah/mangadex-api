use serde::{Deserialize};
use url::Url;

/// Related links for a manga.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaLinks {
    /// Amazon Product URL.
    ///
    /// Stored as full URL.
    ///
    /// # Examples
    ///
    /// - `https://www.amazon.co.jp/gp/product/B074CHFLT2`
    #[serde(rename = "amz")]
    pub amazon: Option<Url>,

    /// AniList ID.
    ///
    /// `https://anilist.co/manga/{id}`
    ///
    /// # Examples
    ///
    /// - `112847`
    #[serde(rename = "al")]
    pub anilist: Option<String>,

    /// Anime-Planet slug.
    ///
    /// `https://www.anime-planet.com/manga/{slug}`
    ///
    /// # Examples
    ///
    /// - `age`
    /// - `under-grand-hotel`
    #[serde(rename = "ap")]
    pub anime_planet: Option<String>,

    /// BookWalker URI.
    ///
    /// Stored has "series/{id}".
    ///
    /// `https://bookwalker.jp/{slug}`
    ///
    /// # Examples
    ///
    /// - `series/289459`
    #[serde(rename = "bw")]
    pub book_walker: Option<BookWalker>,
    /// CDJapan URL.
    ///
    /// Stored as full URL.
    ///
    /// # Examples
    ///
    /// - `http://www.cdjapan.co.jp/product/NEOBK-1963980`
    /// - `http://www.cdjapan.co.jp/searches?term.cat_id=UD-14-06-03&page=&agg_use=cat_ids_hierarchal_treeish_foldable&term.media_format=&f=major&q=%E7%B5%B6%E5%AF%BE%E3%81%AB%E3%81%A8%E3%81%8D%E3%82%81%E3%81%84%E3%81%A6%E3%81%AF%E3%81%84%E3%81%91%E3%81%AA%E3%81%84%EF%BC%81&f=major&q=&f=major&q=&order=scoreboost_cdj&range.rel=&range.sale_price=&term.caption=&term.audio_language=`
    #[serde(rename = "cdj")]
    pub cd_japan: Option<String>,

    /// EbookJapan URL.
    ///
    /// Stored as full URL.
    ///
    /// # Examples
    ///
    /// - `https://ebookjapan.yahoo.co.jp/books/444727/A001841690/`
    /// - `https://www.ebookjapan.jp/ebj/371654/`
    #[serde(rename = "ebj")]
    pub ebook_japan: Option<Url>,

    /// Official English URL.
    ///
    /// Stored as full URL, official English-licenced URL.
    ///
    /// # Examples
    ///
    /// - `https://kodanshacomics.com/series/we-must-never-fall-in-love/`
    #[serde(rename = "engtl")]
    pub english_translation: Option<String>,

    /// Kitsu ID.
    ///
    /// One of:
    ///
    /// - `https://kitsu.io/api/edge/manga/{id}`
    /// - `https://kitsu.io/api/edge/manga?filter[slug]={slug}`
    ///
    /// If integer, use id version of the URL, otherwise use slug one
    ///
    /// # Examples
    ///
    /// - `23040`
    #[serde(rename = "kt")]
    pub kitsu: Option<String>,

    /// MangaUpdates ID.
    ///
    /// `https://www.mangaupdates.com/series.html?id={id}`
    ///
    /// # Examples
    ///
    /// - `157722`
    #[serde(rename = "mu")]
    pub manga_updates: Option<MangaUpdates>,

    /// MyAnimeList ID.
    ///
    /// `https://myanimelist.net/manga/{id}`
    ///
    /// # Examples
    ///
    /// - `12648`
    #[serde(rename = "mal")]
    pub my_anime_list: Option<MyAnimeList>,

    /// NovelUpdates slug.
    ///
    /// `https://www.novelupdates.com/series/{slug}`
    ///
    /// # Examples
    ///
    /// - `an-active-hunter-in-hokkaido-has-been-thrown-into-a-different-world`
    #[serde(rename = "nu")]
    pub novel_updates: Option<NovelUpdates>,

    /// Raw URL.
    ///
    /// Stored as full URL, untranslated stuff URL (original language).
    ///
    /// # Examples
    ///
    /// - `https://manga.bilibili.com/m/detail/mc29443?from=manga_index`
    /// - `https://www.sunday-webry.com/detail-yoru.php?title_id=1282c`
    pub raw: Option<Url>,
}

impl MangaLinks {
    /// Check if there are no available links.
    ///
    /// This returns `false` if at least one of the fields is `Some`.
    pub(crate) fn has_no_links(&self) -> bool {
        self.amazon.is_none()
            && self.anilist.is_none()
            && self.anime_planet.is_none()
            && self.book_walker.is_none()
            && self.cd_japan.is_none()
            && self.ebook_japan.is_none()
            && self.english_translation.is_none()
            && self.kitsu.is_none()
            && self.manga_updates.is_none()
            && self.my_anime_list.is_none()
            && self.novel_updates.is_none()
            && self.raw.is_none()
    }
}

/// BookWalker URI.
///
/// Example: "`series/91701`".
#[derive(Clone, Debug, Deserialize, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BookWalker(pub String);

impl std::fmt::Display for BookWalker {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&*format!("https://bookwalker.jp/{}", self.0))
    }
}

/// MangaUpdates ID.
///
/// Example: "`132515`".
#[derive(Clone, Debug, Deserialize, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaUpdates(pub String);

impl std::fmt::Display for MangaUpdates {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&*format!(
            "https://www.mangaupdates.com/series.html?id={}",
            self.0
        ))
    }
}

/// MyAnimeList ID.
///
/// Example: "`98436`".
#[derive(Clone, Debug, Deserialize, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MyAnimeList(pub String);

impl std::fmt::Display for MyAnimeList {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&*format!("https://myanimelist.net/manga/{}", self.0))
    }
}

/// NovelUpdates slug.
///
/// Example: "`novel-updates`".
#[derive(Clone, Debug, Deserialize, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NovelUpdates(pub String);

impl std::fmt::Display for NovelUpdates {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&*format!("https://www.novelupdates.com/series/{}/", self.0))
    }
}
