//! Builder for the manga view endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let manga_id = Uuid::new_v4();
//! let manga_res = client
//!     .manga()
//!     .id(manga_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("manga view: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
pub struct GetManga {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/manga/{}", manga_id),
    #[query] GetManga,
    #[flatten_result] MangaResponse,
    GetMangaBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::RelatedAttributes;
    use mangadex_api_types::{
        MangaDexDateTime, MangaRelation, ReferenceExpansionResource, RelationshipType,
    };

    #[tokio::test]
    async fn get_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "manga",
                "attributes": {
                    "title": {
                        "en": "Test Manga"
                    },
                    "altTitles": [],
                    "description": {},
                    "isLocked": false,
                    "links": {},
                    "originalLanguage": "ja",
                    "lastVolume": "1",
                    "lastChapter": "1",
                    "publicationDemographic": "shoujo",
                    "status": "completed",
                    "year": 2021,
                    "contentRating": "safe",
                    "chapterNumbersResetOnNewVolume": true,
                    "availableTranslatedLanguages": ["en"],
                    "tags": [],
                    "state": "published",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": [
                    {
                        "id": "a3219a4f-73c0-4213-8730-05985130539a",
                        "type": "manga",
                        "related": "side_story",
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.manga().id(manga_id).get().send().await?;

        assert_eq!(res.data.relationships[0].type_, RelationshipType::Manga);
        assert_eq!(
            res.data.relationships[0].related,
            Some(MangaRelation::SideStory)
        );
        assert!(res.data.relationships[0].attributes.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn get_manga_handles_reference_expansion() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "manga",
                "attributes": {
                    "title": {
                        "en": "Test Manga"
                    },
                    "altTitles": [],
                    "description": {},
                    "isLocked": false,
                    "links": {},
                    "originalLanguage": "ja",
                    "lastVolume": "1",
                    "lastChapter": "1",
                    "publicationDemographic": "shoujo",
                    "status": "completed",
                    "year": 2021,
                    "contentRating": "safe",
                    "chapterNumbersResetOnNewVolume": true,
                    "availableTranslatedLanguages": ["en"],
                    "tags": [],
                    "state": "published",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": [
                    {
                        "id": "fc343004-569b-4750-aba0-05ab35efc17c",
                        "type": "author",
                        "attributes": {
                            "name": "Hologfx",
                            "imageUrl": null,
                            "biography": [],
                            "twitter": null,
                            "pixiv": null,
                            "melonBook": null,
                            "fanBox": null,
                            "booth": null,
                            "nicoVideo": null,
                            "skeb": null,
                            "fantia": null,
                            "tumblr": null,
                            "youtube": null,
                            "website": null,
                            "createdAt": "2021-04-19T21:59:45+00:00",
                            "updatedAt": "2021-04-19T21:59:45+00:00",
                            "version": 1
                        }
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .id(manga_id)
            .get()
            .include(&ReferenceExpansionResource::Author)
            .send()
            .await?;

        assert_eq!(res.data.relationships[0].type_, RelationshipType::Author);
        assert!(res.data.relationships[0].related.is_none());
        match res.data.relationships[0].attributes.as_ref().unwrap() {
            RelatedAttributes::Author(author) => assert_eq!(author.name, "Hologfx".to_string()),
            _ => panic!("Expected author RelatedAttributes"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_manga_handles_null_available_translated_languages_element_value(
    ) -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "manga",
                "attributes": {
                    "title": {
                        "en": "Komi-san wa Komyushou Desu."
                    },
                    "altTitles": [
                        {
                            "ja-ro": "Comi-san ha Comyusho Desu."
                        },
                        {
                            "en": "Komi Can't Communicate"
                        },
                        {
                            "en": "Komi-san Can't Communicate."
                        },
                        {
                            "en": "Komi-san Has a Communication Disorder."
                        },
                        {
                            "ja-ro": "Komi-san wa Komyushou Desu"
                        },
                        {
                            "ja-ro": "Komi-san wa, Communication Shougai desu."
                        },
                        {
                            "ja-ro": "Komi-san wa, Comyushou desu."
                        },
                        {
                            "ja-ro": "Komi-san wa, Komyushou desu."
                        },
                        {
                            "en": "Miss Komi Is Bad at Communication."
                        },
                        {
                            "ru": "У Коми-сан проблемы с общением"
                        },
                        {
                            "th": "โฉมงามพูดไม่เก่งกับผองเพื่อนไม่เต็มเต็ง"
                        },
                        {
                            "ja": "古見さんは、コミュ症です。"
                        },
                        {
                            "zh": "古見同學有交流障礙症"
                        },
                        {
                            "ko": "코미 양은, 커뮤증이에요"
                        }
                    ],
                    "description": {
                        "en": "Komi-san is a beautiful and admirable girl that no one can take their eyes off of. Almost the whole school sees her as the cold beauty that's out of their league, but Tadano Hitohito knows the truth: she's just really bad at communicating with others.\n\nKomi-san, who wishes to fix this bad habit of hers, tries to improve it with the help of Tadano-kun by achieving her goal of having 100 friends.",
                        "pl": "Komi-san jest piękną i godną podziwu dziewczyną, od której nikt nie może oderwać oczu. Prawie cała szkoła postrzega ją jako zimne piękno, które jest poza ich zasięgiem, ale Tadano Hitohito zna prawdę: młoda piękność po prostu źle komunikuje się z innymi. Komi-san, chce to zmienić, a ma jej w tym pomóc Tadano.",
                        "pt-br": "Komi-san é uma bela e admirável garota que ninguém consegue tirar os olhos. Quase todos da escola a veem como alguém fora do alcance, mas Tadano Shigeo sabe a verdade: **ela apenas não sabe como se comunicar com os outras pessoas**. Komi-san, que deseja corrigir este mau hábito dela, tenta melhorar com a ajuda do Tadano-kun..."
                    },
                    "isLocked": true,
                    "links": {
                        "al": "97852",
                        "ap": "komi-cant-communicate",
                        "bw": "series/129153",
                        "kt": "37855",
                        "mu": "127281",
                        "amz": "https://www.amazon.co.jp/gp/product/B07CBD8DKM",
                        "cdj": "http://www.cdjapan.co.jp/product/NEOBK-1985640",
                        "ebj": "https://ebookjapan.yahoo.co.jp/books/382444/",
                        "mal": "99007",
                        "raw": "https://websunday.net/rensai/komisan/",
                        "engtl": "https://www.viz.com/komi-can-t-communicate"
                    },
                    "originalLanguage": "ja",
                    "lastVolume": "",
                    "lastChapter": "",
                    "publicationDemographic": "shounen",
                    "status": "ongoing",
                    "year": 2016,
                    "contentRating": "safe",
                    "tags": [
                        {
                            "id": "423e2eae-a7a2-4a8b-ac03-a8351462d71d",
                            "type": "tag",
                            "attributes": {
                                "name": {
                                    "en": "Romance"
                                },
                                "description": [],
                                "group": "genre",
                                "version": 1
                            },
                            "relationships": []
                        },
                        {
                            "id": "4d32cc48-9f00-4cca-9b5a-a839f0764984",
                            "type": "tag",
                            "attributes": {
                                "name": {
                                    "en": "Comedy"
                                },
                                "description": [],
                                "group": "genre",
                                "version": 1
                            },
                            "relationships": []
                        },
                        {
                            "id": "caaa44eb-cd40-4177-b930-79d3ef2afe87",
                            "type": "tag",
                            "attributes": {
                                "name": {
                                  "en": "School Life"
                                },
                                "description": [],
                                "group": "theme",
                                "version": 1
                            },
                            "relationships": []
                        },
                        {
                            "id": "e5301a23-ebd9-49dd-a0cb-2add944c7fe9",
                            "type": "tag",
                            "attributes": {
                                "name": {
                                  "en": "Slice of Life"
                                },
                                "description": [],
                                "group": "genre",
                                "version": 1
                            },
                            "relationships": []
                        }
                    ],
                    "state": "published",
                    "chapterNumbersResetOnNewVolume": false,
                    "createdAt": "2018-11-22T23:31:37+00:00",
                    "updatedAt": "2022-02-13T22:49:56+00:00",
                    "version": 85,
                    "availableTranslatedLanguages": [
                        "pt-br",
                        "cs",
                        "ru",
                        "en",
                        "fa",
                        "tr",
                        "fr",
                        "pl",
                        "mn",
                        "es-la",
                        "id",
                        "it",
                        "hi",
                        "tl",
                        "hu",
                        "de",
                        "ro",
                        "nl",
                        null
                    ]
                },
                "relationships": [
                    {
                        "id": "4218b1ee-cde4-44dc-84c7-d9a794a7e56d",
                        "type": "author"
                    },
                    {
                        "id": "4218b1ee-cde4-44dc-84c7-d9a794a7e56d",
                        "type": "artist"
                    },
                    {
                        "id": "9324d3c0-d90d-4f3e-b79b-866029b721a7",
                        "type": "cover_art"
                    },
                    {
                        "id": "2917e1b1-06c0-45fe-b30b-6688d83859b2",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "3e8df40e-e2b3-4336-987b-f3e52d00ce5f",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "60e5c222-f0aa-4f14-baba-b18207321d5e",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "82478f68-943e-4391-b445-f2f9b0007b95",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "973de049-748a-4446-98b9-dfea826f61a5",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "cb655d77-f369-4a06-9a35-b38c00f34e9b",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "d6448e6b-4409-4380-b74e-1629c6d1d1a7",
                        "type": "manga",
                        "related": "doujinshi"
                    },
                    {
                        "id": "fb569d12-1e00-47e3-86cd-793b4eae715c",
                        "type": "manga",
                        "related": "colored"
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.manga().id(manga_id).get().send().await?;

        // `null` should not be included in the sequence.
        assert_eq!(res.data.attributes.available_translated_languages.len(), 18);

        Ok(())
    }
}
