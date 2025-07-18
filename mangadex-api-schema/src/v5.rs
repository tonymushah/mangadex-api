pub mod api_client;
pub mod at_home_server;
pub mod auth_tokens;
pub mod author;
pub mod chapter;
pub mod check_token_response;
pub mod check_username_available;
pub mod cover;
pub mod custom_list;
mod exports_types;
pub mod forum_thread;
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
pub mod oauth;
pub mod ratings;
pub mod refresh_token_response;
pub mod report;
pub mod scanlation_group;
pub mod settings_template;
pub mod statistics;
pub mod tag;
pub mod upload_required_approval;
pub mod upload_session;
pub mod upload_session_file;
pub mod user;
pub mod user_history;
pub mod user_report;
pub mod user_settings;

pub use self::exports_types::*;
use std::collections::HashMap;

use mangadex_api_types as types;
use serde::Deserialize;
use uuid::Uuid;

use types::{
    Language, MangaDexDateTime, MangaRelation, RelationshipType, ResponseType, ResultType,
};

use crate::error::RelationshipConversionError;
pub(crate) use crate::{ApiObject, ApiObjectNoRelationships};

// TODO: Find a way to reduce the boilerplate for this.
// `struct-variant` (https://docs.rs/struct-variant) is a potential candidate for this.
#[derive(Debug, Deserialize, Clone)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
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

impl TryFrom<Relationship> for ApiObjectNoRelationships<MangaAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::Manga {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::Manga,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::Manga(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::Manga,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::Manga,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<ChapterAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::Chapter {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::Chapter,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::Chapter(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::Chapter,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::Chapter,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<CoverAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::CoverArt {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::CoverArt,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::CoverArt(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::CoverArt,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::CoverArt,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<AuthorAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if !(value.type_ == RelationshipType::Author || value.type_ == RelationshipType::Artist) {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::Author,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::Author(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::Author,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::Author,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<ScanlationGroupAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::ScanlationGroup {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::ScanlationGroup,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::ScanlationGroup(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::ScanlationGroup,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::ScanlationGroup,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<TagAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::Tag {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::Tag,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::Tag(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::Tag,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::Tag,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<UserAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if !(value.type_ == RelationshipType::User
            || value.type_ == RelationshipType::Member
            || value.type_ == RelationshipType::Leader)
        {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::User,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::User(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::User,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::User,
            ))
        }
    }
}

impl TryFrom<Relationship> for ApiObjectNoRelationships<CustomListAttributes> {
    type Error = RelationshipConversionError;

    fn try_from(value: Relationship) -> Result<Self, Self::Error> {
        if value.type_ != RelationshipType::CustomList {
            return Err(RelationshipConversionError::InvalidInputRelationshipType {
                input: RelationshipType::CustomList,
                inner: value.type_,
            });
        }
        if let Some(RelatedAttributes::CustomList(attributes)) = value.attributes {
            Ok(Self {
                id: value.id,
                type_: RelationshipType::CustomList,
                attributes,
            })
        } else {
            Err(RelationshipConversionError::AttributesNotFound(
                RelationshipType::CustomList,
            ))
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct Relationship {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_: RelationshipType,
    /// Related Manga type.
    ///
    /// <https://api.mangadex.org/docs/static-data/#manga-related-enum>
    ///
    /// This is only present for a Manga entity and a Manga relationship.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub related: Option<MangaRelation>,
    /// Contains object attributes for the type.
    ///
    /// Present if [Reference Expansion](https://api.mangadex.org/docs/reference-expansion/) is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes: Option<RelatedAttributes>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct Results<T> {
    #[serde(default)]
    pub result: ResultType,
    pub response: ResponseType,
    pub data: Vec<T>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
}

impl<T> Default for Results<T> {
    fn default() -> Self {
        Self {
            result: ResultType::Ok,
            response: ResponseType::Collection,
            data: Vec::default(),
            limit: 0,
            offset: 0,
            total: 0,
        }
    }
}

pub type LocalizedString = HashMap<Language, String>;

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex currently returns an empty array when the localized string field isn't present.
///
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod localizedstring_array_or_map {
    use std::collections::HashMap;

    use super::LocalizedString;
    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    #[cfg(feature = "serialize")]
    use serde::ser::{Serialize, Serializer};

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
    #[cfg(feature = "serialize")]
    pub fn serialize<S>(to_use: &LocalizedString, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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
    use super::manga_aggregate::VolumeAggregate;
    #[cfg(feature = "serialize")]
    use serde::Serialize;
    use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
    #[cfg(feature = "serialize")]
    use serde::ser::Serializer;
    use std::collections::BTreeMap;
    #[cfg(feature = "serialize")]
    use std::collections::HashMap;

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
    #[cfg(feature = "serialize")]
    #[allow(dead_code)]
    pub fn serialize<S>(
        to_use: &VolumeAggregateCollection,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use super::manga_aggregate::VolumeAggregateSer;

        let mut volumes: HashMap<String, VolumeAggregateSer> = HashMap::new();
        for volume in to_use {
            volumes.insert(volume.volume.clone(), Into::into(volume.clone()));
        }
        volumes.serialize(serializer)
    }
}

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex sometimes returns an array instead of a JSON object for the chapter aggregate field.
///
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod chapter_aggregate_array_or_map {
    #[cfg(feature = "serialize")]
    use serde::Serialize;
    use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
    #[cfg(feature = "serialize")]
    use serde::ser::Serializer;
    use std::collections::BTreeMap;

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
    #[cfg(feature = "serialize")]
    pub fn serialize<S>(
        to_use: &ChapterAggregateCollection,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::collections::HashMap;

        let mut chapters: HashMap<String, ChapterAggregate> = HashMap::new();
        for chapter in to_use {
            chapters.insert(chapter.chapter.clone(), chapter.clone());
        }
        chapters.serialize(serializer)
    }
}

/// Originally a Deserializer helper to handle JSON array or object types.
///
/// MangaDex sometimes returns an array instead of a JSON object for the `links` field for `MangaAttributes`.
///
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod manga_links_array_or_struct {
    use crate::v5::MangaLinks;
    #[cfg(feature = "serialize")]
    use serde::Serialize;
    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    #[cfg(feature = "serialize")]
    use serde::ser::Serializer;

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
    #[cfg(feature = "serialize")]
    pub fn serialize<S>(to_use: &Option<MangaLinks>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match to_use {
            None => serializer.serialize_none(),
            Some(data) => data.serialize(serializer),
        }
    }
}

/// Originally a Deserializer for an array of languages, discarding elements that are `null`.
///
/// The Serializer was added in 0.2.0 for pratical and necessities reason
pub(crate) mod language_array_or_skip_null {
    use mangadex_api_types::Language;
    #[cfg(feature = "serialize")]
    use serde::Serialize;
    use serde::de::{Deserializer, SeqAccess, Visitor};
    #[cfg(feature = "serialize")]
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
    #[cfg(feature = "serialize")]
    pub fn serialize<S>(to_use: &Vec<Language>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        to_use.serialize(serializer)
    }
}

pub fn mangadex_datetime_serialize<S>(
    datetime: &MangaDexDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(datetime.to_string().as_str())
}

pub fn mangadex_datetime_serialize_option<S>(
    datetime: &Option<MangaDexDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(d) = datetime {
        serializer.serialize_str(d.to_string().as_str())
    } else {
        serializer.serialize_none()
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use mangadex_api_types::MangaDexDateTime;

    #[derive(Serialize, Deserialize, Default)]
    struct TestStruct {
        #[serde(serialize_with = "crate::v5::mangadex_datetime_serialize")]
        date: MangaDexDateTime,
    }

    #[tokio::test]
    async fn mangadex_datetime_serialize_test() {
        let test: TestStruct = Default::default();
        println!("{}", serde_json::to_string_pretty(&test).unwrap());
    }
}
