use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Clone, Debug, Copy, Default)]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Comments {
    pub thread_id: u32,
    pub replies_count: u32,
}

impl TryInto<Url> for Comments {
    type Error = url::ParseError;

    fn try_into(self) -> Result<Url, Self::Error> {
        Url::parse(format!("https://forums.mangadex.org/threads/{}", self.thread_id).as_str())
    }
}
