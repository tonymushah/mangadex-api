use serde::{Deserialize, Serialize};

/// Used in the `related` field of a Manga relationships.
///
/// <https://api.mangadex.org/docs/static-data/#manga-related-enum>
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, Eq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[non_exhaustive]
pub enum MangaRelation {
    /// The original work this spin-off manga has been adapted from.
    AdaptedFrom,
    /// An alternative take of the story in this manga.
    AlternateStory,
    /// A different version of this manga with no other specific distinction.
    AlternateVersion,
    /// The original work this self-published derivative manga is based on.
    BasedOn,
    /// A colored variant of this manga.
    Colored,
    /// A self-published derivative work based on this manga.
    Doujinshi,
    /// The original narrative this manga is based on.
    MainStory,
    /// A monochrome variant of this manga.
    #[default]
    Monochrome,
    /// The previous entry in the same series.
    Prequel,
    /// The original version of this manga before its official serialization.
    Preserialization,
    /// A manga based on the same intellectual property as this manga.
    SameFranchise,
    /// The next entry in the same series.
    Sequel,
    /// The official serialization of this manga.
    Serialization,
    /// A manga taking place in the same fictional world as this manga.
    SharedUniverse,
    /// A side work contemporaneous with the narrative of this manga.
    SideStory,
    /// An official derivative work based on this manga.
    SpinOff,
}

impl std::fmt::Display for MangaRelation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::AdaptedFrom => "Adapted From",
            Self::AlternateStory => "Alternate Story",
            Self::AlternateVersion => "Alternate Version",
            Self::BasedOn => "Based On",
            Self::Colored => "Colored",
            Self::Doujinshi => "Doujinshi",
            Self::MainStory => "Main Story",
            Self::Monochrome => "Monochrome",
            Self::Prequel => "Prequel",
            Self::Preserialization => "Preserialization",
            Self::SameFranchise => "Same Franchise",
            Self::Sequel => "Sequel",
            Self::Serialization => "Serialization",
            Self::SharedUniverse => "Shared Universe",
            Self::SideStory => "Side Story",
            Self::SpinOff => "Spin Off",
        };
        fmt.write_str(name)
    }
}
