pub mod at_home_server;
pub mod auth_tokens;
pub mod author;
pub mod chapter;
pub mod check_token_response;
pub mod check_username_available;
pub mod cover;
pub mod custom_list;
pub mod error;
pub mod is_following_response;
pub mod legacy_id_mapping;
pub mod login_response;
pub mod manga;
pub mod manga_aggregate;
pub mod manga_links;
pub mod manga_read_markers;
pub mod manga_reading_status;
pub mod manga_reading_statuses;
pub mod manga_relation;
pub mod ratings;
pub mod refresh_token_response;
pub mod report;
pub mod scanlation_group;
pub mod statistics;
pub mod tag;
pub mod upload_session;
pub mod upload_session_file;
pub mod user;
pub mod user_report;
pub mod user_settings;

use std::collections::HashMap;

use mangadex_api_types as types;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ApiData, ApiObject, ApiObjectNoRelationships};
pub use at_home_server::AtHomeServer;
pub use auth_tokens::AuthTokens;
pub use author::AuthorAttributes;
pub use chapter::ChapterAttributes;
pub use check_token_response::CheckTokenResponse;
pub use check_username_available::CheckUsernameAvailableResponse;
pub use cover::CoverAttributes;
pub use custom_list::CustomListAttributes;
pub use is_following_response::IsFollowingResponse;
pub use legacy_id_mapping::LegacyMappingIdAttributes;
pub use login_response::LoginResponse;
pub use manga::MangaAttributes;
pub use manga_aggregate::MangaAggregate;
pub use manga_links::MangaLinks;
pub use manga_read_markers::{MangaReadMarkers, UngroupedMangaReadMarkers};
pub use manga_reading_status::MangaReadingStatus;
pub use manga_reading_statuses::MangaReadingStatuses;
pub use manga_relation::MangaRelationAttributes;
pub use ratings::RatingsList;
pub use refresh_token_response::RefreshTokenResponse;
pub use report::ReportReasonAttributes;
pub use scanlation_group::ScanlationGroupAttributes;
pub use statistics::manga::MangaStatisticsObject;
pub use tag::TagAttributes;
pub use types::error::schema::MangaDexErrorResponse;
use types::error::Result;
use types::{Language, MangaRelation, RelationshipType, ResponseType};
pub use upload_session::UploadSessionResponse;
pub use upload_session_file::{UploadSessionFileAttributes, UploadSessionFileData};
pub use user::UserAttributes;
pub use user_report::UserReportAttributes;
pub use user_settings::UserSettingsAttributes;

pub type AtHomeServerResponse = Result<AtHomeServer>;

pub type AuthorObject = ApiObject<AuthorAttributes>;
pub type AuthorData = ApiData<AuthorObject>;
pub type AuthorResponse = Result<AuthorData>;
pub type AuthorListResponse = Result<Results<AuthorObject>>;

pub type ChapterObject = ApiObject<ChapterAttributes>;
pub type ChapterData = ApiData<ChapterObject>;
pub type ChapterResponse = Result<ChapterData>;
pub type ChapterListResponse = Result<Results<ChapterObject>>;

pub type CoverObject = ApiObject<CoverAttributes>;
pub type CoverData = ApiData<CoverObject>;
pub type CoverResponse = Result<CoverData>;
pub type CoverListResponse = Result<Results<CoverObject>>;

pub type CustomListObject = ApiObject<CustomListAttributes>;
pub type CustomListData = ApiData<CustomListObject>;
pub type CustomListResponse = Result<CustomListData>;
pub type CustomListListResponse = Result<Results<CustomListObject>>;

pub type GroupObject = ApiObject<ScanlationGroupAttributes>;
pub type GroupData = ApiData<GroupObject>;
pub type GroupResponse = Result<GroupData>;
pub type GroupListResponse = Result<Results<GroupObject>>;

pub type IdMappingObject = ApiObject<LegacyMappingIdAttributes>;
pub type IdMappingData = ApiData<IdMappingObject>;
pub type IdMappingListResponse = Result<Results<IdMappingObject>>;

pub type MangaObject = ApiObject<MangaAttributes>;
pub type MangaData = ApiData<MangaObject>;
pub type MangaResponse = Result<MangaData>;
pub type MangaListResponse = Result<Results<MangaObject>>;

pub type MangaAggregateResponse = Result<MangaAggregate>;

pub type UngroupedMangaReadMarkersResponse = Result<UngroupedMangaReadMarkers>;
pub type MangaReadMarkersResponse = Result<MangaReadMarkers>;

pub type MangaReadingStatusResponse = Result<MangaReadingStatus>;
pub type MangaReadingStatusesResponse = Result<MangaReadingStatuses>;

pub type MangaRelationObject = ApiObject<MangaRelationAttributes>;
pub type MangaRelationListResponse = Result<Results<MangaRelationObject>>;

pub type MangaStatisticsResponse = Result<MangaStatisticsObject>;

pub type RatingsResponse = Result<RatingsList>;

pub type ReportReasonObject = ApiObjectNoRelationships<ReportReasonAttributes>;
pub type ReportReasonListResponse = Result<Results<ReportReasonObject>>;

pub type TagObject = ApiObject<TagAttributes>;
pub type TagData = ApiData<TagObject>;
pub type TagResponse = Result<TagData>;
pub type TagListResponse = Result<Results<TagObject>>;

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileResponse = Result<UploadSessionFileData<UploadSessionFileObject>>;

pub type UserObject = ApiObject<UserAttributes>;
pub type UserData = ApiData<UserObject>;
pub type UserResponse = Result<UserData>;
pub type UserListResponse = Result<Results<UserObject>>;

pub type UserReportsObject = ApiObject<UserReportAttributes>;
pub type UserReportsData = ApiData<UserReportsObject>;
pub type UserReportsListResponse = Result<Results<UserReportsObject>>;

pub type UserSettingsResponse = Result<UserSettingsAttributes>;

// TODO: Find a way to reduce the boilerplate for this.
// `struct-variant` (https://docs.rs/struct-variant) is a potential candidate for this.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum RelatedAttributes {
    /// Manga resource.
    Manga(MangaAttributes),
    /// Chapter resource.
    Chapter(ChapterAttributes),
    /// A Cover Art for a manga.
    ///
    /// On manga resources, only one cover art resource relation is returned,
    /// marking the primary cover if there are more than one. By default, this will be the latest
    /// volume's cover art. To see all the covers for a given manga, use the cover search endpoint.
    CoverArt(CoverAttributes),
    /// Author resource.
    Author(AuthorAttributes),
    /// ScanlationGroup resource.
    ScanlationGroup(ScanlationGroupAttributes),
    /// Tag resource.
    Tag(TagAttributes),
    /// User resource.
    User(UserAttributes),
    /// CustomList resource.
    CustomList(CustomListAttributes),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Relationship {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_: RelationshipType,
    /// Related Manga type.
    ///
    /// <https://api.mangadex.org/docs/static-data/#manga-related-enum>
    ///
    /// This is only present for a Manga entity and a Manga relationship.
    pub related: Option<MangaRelation>,
    /// Contains object attributes for the type.
    ///
    /// Present if [Reference Expansion](https://api.mangadex.org/docs/reference-expansion/) is applied.
    pub attributes: Option<RelatedAttributes>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Results<T> {
    pub response: ResponseType,
    pub data: Vec<T>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
}

pub type LocalizedString = HashMap<Language, String>;

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex currently returns an empty array when the localized string field isn't present.
/// 
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod localizedstring_array_or_map {
    use std::collections::HashMap;

    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};
    use super::LocalizedString;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<LocalizedString, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = LocalizedString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("array or object")
            }

            fn visit_seq<A>(self, mut _seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok(HashMap::new())
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let de = serde::de::value::MapAccessDeserializer::new(map);
                let helper = LocalizedString::deserialize(de)?;
                Ok(helper)
            }
        }

        deserializer.deserialize_any(V)
    }
    pub fn serialize<S>(to_use : &LocalizedString ,serializer : S) -> Result<S::Ok, S::Error>
        where 
            S : Serializer
    {
        to_use.serialize(serializer)
    }
}

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex sometimes returns an array instead of a JSON object for the volume aggregate field.
/// 
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod volume_aggregate_array_or_map {
    use std::collections::BTreeMap;

    use serde::Serialize;
    use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
    use serde::ser::Serializer;
    use super::manga_aggregate::VolumeAggregate;

    type VolumeAggregateCollection = Vec<VolumeAggregate>;

    const PAD_WIDTH: usize = 5;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<VolumeAggregateCollection, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = VolumeAggregateCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("array or object")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut volumes = Vec::new();

                while let Some(volume) = seq.next_element::<VolumeAggregate>()? {
                    volumes.push(volume);
                }

                Ok(volumes)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                // Temporary collection to sort the results because serde doesn't seem to iterate
                // through the map in the order they appear.
                let mut sorting_map = BTreeMap::new();

                while let Some((volume_number, volume)) =
                    map.next_entry::<String, VolumeAggregate>()?
                {
                    let volume_number = if volume_number.contains('.') {
                        match volume_number.parse::<f64>() {
                            Ok(_) => {
                                //
                                let (i, f) = volume_number.split_once('.').unwrap();
                                let i = i.parse::<i32>().unwrap();
                                // Pad the whole number part so that it is sorted correctly with the
                                // other keys.
                                format!("{i:0PAD_WIDTH$}.{f}")
                            }
                            Err(_) => volume_number,
                        }
                    } else {
                        match volume_number.parse::<i32>() {
                            Ok(n) => format!("{n:0PAD_WIDTH$}"),
                            Err(_) => volume_number,
                        }
                    };
                    sorting_map.insert(volume_number, volume);
                }

                Ok(sorting_map.values().cloned().collect())
            }
        }

        deserializer.deserialize_any(V)
    }
    pub fn serialize<S>(to_use : &VolumeAggregateCollection, serializer : S) -> Result<S::Ok, S::Error>
    where
        S : Serializer
    {
        to_use.serialize(serializer)
    }
}

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex sometimes returns an array instead of a JSON object for the chapter aggregate field.
/// 
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod chapter_aggregate_array_or_map {
    use std::collections::BTreeMap;
    use serde::Serialize;
    use serde::ser::Serializer;
    use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};

    use super::manga_aggregate::ChapterAggregate;

    const PAD_WIDTH: usize = 5;

    type ChapterAggregateCollection = Vec<ChapterAggregate>;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ChapterAggregateCollection, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = ChapterAggregateCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("array or object")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut chapters = Vec::new();

                while let Some(chapter) = seq.next_element::<ChapterAggregate>()? {
                    chapters.push(chapter);
                }

                Ok(chapters)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                // Temporary collection to sort the results because serde doesn't seem to iterate
                // through the map in the order they appear.
                let mut sorting_map = BTreeMap::new();

                while let Some((chapter_number, chapter)) =
                    map.next_entry::<String, ChapterAggregate>()?
                {
                    let chapter_number = if chapter_number.contains('.') {
                        match chapter_number.parse::<f64>() {
                            Ok(_) => {
                                //
                                let (i, f) = chapter_number.split_once('.').unwrap();
                                let i = i.parse::<i32>().unwrap();
                                // Pad the whole number part so that it is sorted correctly with the
                                // other keys.
                                format!("{i:0PAD_WIDTH$}.{f}")
                            }
                            Err(_) => chapter_number,
                        }
                    } else {
                        match chapter_number.parse::<i32>() {
                            Ok(n) => format!("{n:0PAD_WIDTH$}"),
                            Err(_) => chapter_number,
                        }
                    };
                    sorting_map.insert(chapter_number, chapter);
                }

                Ok(sorting_map.values().cloned().collect())
            }
        }

        deserializer.deserialize_any(V)
    }

    pub fn serialize<S>(to_use : &ChapterAggregateCollection, serializer : S) -> Result<S::Ok, S::Error>
    where
        S : Serializer
    {
        to_use.serialize(serializer)
    }
}

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex sometimes returns an array instead of a JSON object for the `links` field for `MangaAttributes`.
/// 
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod manga_links_array_or_struct {
    use serde::Serialize;
    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    use serde::ser::Serializer;
    use crate::v5::MangaLinks;

    /// Deserialize a `MangaLinks` from a JSON value or none.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<MangaLinks>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionMangaLinksVisitor;

        impl<'de> Visitor<'de> for OptionMangaLinksVisitor {
            type Value = Option<MangaLinks>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("some or none")
            }

            /// Deserialize a `MangaLinks` from none.
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            /// Deserialize a `MangaLinks` from a JSON value.
            fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let manga_links = d.deserialize_any(MangaLinksVisitor)?;

                let manga_links = if manga_links.has_no_links() {
                    None
                } else {
                    Some(manga_links)
                };

                Ok(manga_links)
            }

            /// Deserialize a `MangaLinks` from none (`null`).
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        struct MangaLinksVisitor;

        impl<'de> Visitor<'de> for MangaLinksVisitor {
            type Value = MangaLinks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("array or map")
            }

            /// Deserialize a `MangaLinks` from a sequence (array).
            fn visit_seq<A>(self, mut _seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok(Self::Value::default())
            }

            /// Deserialize a `MangaLinks` from a map (JSON object).
            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
                // into a `Deserializer`, allowing it to be used as the input to T's
                // `Deserialize` implementation. T then deserializes itself using
                // the entries from the map visitor.
                Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_option(OptionMangaLinksVisitor)
    }
    pub fn serialize<S>(to_use : &Option<MangaLinks>, serializer : S) -> Result<S::Ok, S::Error> 
    where 
        S : Serializer
    {
        match to_use {
            None => {
                serializer.serialize_none()
            },
            Some(data) => {
                data.serialize(serializer)
            }
        }
    }
}

/// Originally a Deserializer for an array of languages, discarding elements that are `null`.
/// 
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod language_array_or_skip_null {
    use mangadex_api_types::Language;
    use serde::Serialize;
    use serde::de::{Deserializer, SeqAccess, Visitor};
    use serde::ser::Serializer;
    /// Deserialize a `Vec<Language>` from an array of JSON values.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Language>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Vec<Language>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of languages")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut languages = Vec::new();

                // Skip over invalid or `null` languages.
                while let Some(language) = seq.next_element::<Option<Language>>()? {
                    // `language` will be `None` if an element value is `null` from JSON.
                    if let Some(language) = language {
                        languages.push(language);
                    }
                }

                Ok(languages)
            }
        }

        deserializer.deserialize_seq(V)
    }
    pub fn serialize<S>(to_use: &Vec<Language>, serializer : S) -> Result<S::Ok, S::Error>
    where
        S : Serializer
    {
        to_use.serialize(serializer)
    }
}

