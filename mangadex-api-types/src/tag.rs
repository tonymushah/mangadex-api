use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TagGroup {
    Content,
    Format,
    Genre,
    Theme,
}

macro_rules! tags {
    (
        $(
            $( #[$meta:meta] )*
            $tag:ident ($group:ident, $name:literal) => $uuid:literal,
        )*
    ) => {
        /// Enum for serialization to tag UUID.
        #[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
        #[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
        #[cfg_attr(feature = "specta", derive(specta::Type))]
        pub enum Tag {
            $(
                $( #[$meta] )*
                $tag,
            )*
        }

        impl std::fmt::Display for Tag {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(match self {
                    $(
                        Self::$tag => $name,
                    )*
                })
            }
        }

        impl TryFrom<Uuid> for Tag {
            type Error = Error;

            fn try_from(value: Uuid) -> Result<Self, Self::Error> {
                match value.to_string().as_str() {
                    $(
                        $uuid => Ok(Self::$tag),
                    )*
                    _ => Err(Error::ParseError("unexpected tag UUID".to_string())),
                }
            }
        }

        impl From<Tag> for Uuid {
            fn from(value: Tag) -> Self {
                match value {
                    $(
                        Tag::$tag => Uuid::parse_str($uuid).unwrap(),
                    )*
                }
            }
        }

        impl From<Tag> for TagGroup {
            fn from(value: Tag) -> Self {
                match value {
                    $(
                        Tag::$tag => Self::$group,
                    )*
                }
            }
        }
    };
}

tags! {
    Gore(Content, "Gore") => "b29d6a3d-1569-4e7a-8caf-7557bc92cd5d",
    SexualViolence(Content, "Sexual Violence") => "97893a4c-12af-4dac-b6be-0dffb353568e",

    Adaptation(Format, "Adaptation") => "f4122d1c-3b44-44d0-9936-ff7502c39ad3",
    Anthology(Format, "Anthhology") => "51d83883-4103-437c-b4b1-731cb73d786c",
    AwardWinning(Format, "Award Winning") => "0a39b5a1-b235-4886-a747-1d05d216532d",
    Doujinshi(Format, "Doujinshi") => "b13b2a48-c720-44a9-9c77-39c9979373fb",
    FanColored(Format, "Fan Colored") => "7b2ce280-79ef-4c09-9b58-12b7c23a9b78",
    FourKoma(Format, "4-Koma") => "b11fda93-8f1d-4bef-b2ed-8803d3733170",
    FullColor(Format, "Full Color") => "f5ba408b-0e7a-484d-8d49-4e9125ac96de",
    LongStrip(Format, "Long Strip") => "3e2b8dae-350e-4ab8-a8ce-016e844b9f0d",
    OfficialColored(Format, "Official Colored") => "320831a8-4026-470b-94f6-8353740e6f04",
    Oneshot(Format, "Oneshot") => "0234a31e-a729-4e28-9d6a-3f87c4966b9e",
    UserCreated(Format, "User Created") => "891cf039-b895-47f0-9229-bef4c96eccd4",
    WebComic(Format, "Web Comic") => "e197df38-d0e7-43b5-9b09-2842d0c326dd",

    Action(Genre, "Action") => "391b0423-d847-456f-aff0-8b0cfc03066b",
    Adventure(Genre, "Adventure") => "87cc87cd-a395-47af-b27a-93258283bbc6",
    BoysLove(Genre, "Boys' Love") => "5920b825-4181-4a17-beeb-9918b0ff7a30",
    Comedy(Genre, "Comedy") => "4d32cc48-9f00-4cca-9b5a-a839f0764984",
    Crime(Genre, "Crime") => "5ca48985-9a9d-4bd8-be29-80dc0303db72",
    Drama(Genre, "Drama") => "b9af3a63-f058-46de-a9a0-e0c13906197a",
    Fantasy(Genre, "Fantasy") => "cdc58593-87dd-415e-bbc0-2ec27bf404cc",
    GirlsLove(Genre, "Girls' Love") => "a3c67850-4684-404e-9b7f-c69850ee5da6",
    Historical(Genre, "Historical") => "33771934-028e-4cb3-8744-691e866a923e",
    Horror(Genre, "Horror") => "cdad7e68-1419-41dd-bdce-27753074a640",
    Isekai(Genre, "Isekai") => "ace04997-f6bd-436e-b261-779182193d3d",
    MagicalGirls(Genre, "Magical Girls") => "81c836c9-914a-4eca-981a-560dad663e73",
    Mecha(Genre, "Mecha") => "50880a9d-5440-4732-9afb-8f457127e836",
    Medical(Genre, "Medical") => "c8cbe35b-1b2b-4a3f-9c37-db84c4514856",
    Mystery(Genre, "Mystery") => "ee968100-4191-4968-93d3-f82d72be7e46",
    Philosophical(Genre, "Philosophical") => "b1e97889-25b4-4258-b28b-cd7f4d28ea9b",
    Psychological(Genre, "Psychological") => "3b60b75c-a2d7-4860-ab56-05f391bb889c",
    Romance(Genre, "Romance") => "423e2eae-a7a2-4a8b-ac03-a8351462d71d",
    SciFi(Genre, "Sci-Fi") => "256c8bd9-4904-4360-bf4f-508a76d67183",
    SliceOfLife(Genre, "Slice of Life") => "e5301a23-ebd9-49dd-a0cb-2add944c7fe9",
    Sports(Genre, "Sports") => "69964a64-2f90-4d33-beeb-f3ed2875eb4c",
    Superhero(Genre, "Superhero") => "7064a261-a137-4d3a-8848-2d385de3a99c",
    Thriller(Genre, "Thriller") => "07251805-a27e-4d59-b488-f0bfbec15168",
    Tragedy(Genre, "Tragedy") => "f8f62932-27da-4fe4-8ee1-6779a8c5edba",
    Wuxia(Genre, "Wuxia") => "acc803a4-c95a-4c22-86fc-eb6b582d82a2",

    Aliens(Theme, "Aliens") => "e64f6742-c834-471d-8d72-dd51fc02b835",
    Animals(Theme, "Animals") => "3de8c75d-8ee3-48ff-98ee-e20a65c86451",
    Cooking(Theme, "Cooking") => "ea2bc92d-1c26-4930-9b7c-d5c0dc1b6869",
    Crossdressing(Theme, "Crossdressing") => "9ab53f92-3eed-4e9b-903a-917c86035ee3",
    Delinquents(Theme, "Delinquents") => "da2d50ca-3018-4cc0-ac7a-6b7d472a29ea",
    Demons(Theme, "Demons") => "39730448-9a5f-48a2-85b0-a70db87b1233",
    Genderswap(Theme, "Genderswap") => "2bd2e8d0-f146-434a-9b51-fc9ff2c5fe6a",
    Ghosts(Theme, "Ghosts") => "3bb26d85-09d5-4d2e-880c-c34b974339e9",
    Gyaru(Theme, "Gyaru") => "fad12b5e-68ba-460e-b933-9ae8318f5b65",
    Harem(Theme, "Harem") => "aafb99c1-7f60-43fa-b75f-fc9502ce29c7",
    Incest(Theme, "Incest") => "5bd0e105-4481-44ca-b6e7-7544da56b1a3",
    Loli(Theme, "Loli") => "2d1f5d56-a1e5-4d0d-a961-2193588b08ec",
    Mafia(Theme, "Mafia") => "85daba54-a71c-4554-8a28-9901a8b0afad",
    Magic(Theme, "Magic") => "a1f53773-c69a-4ce5-8cab-fffcd90b1565",
    MartialArts(Theme, "Martial Arts") => "799c202e-7daa-44eb-9cf7-8a3c0441531e",
    Military(Theme, "Military") => "ac72833b-c4e9-4878-b9db-6c8a4a99444a",
    Monsters(Theme, "Monsters") => "36fd93ea-e8b8-445e-b836-358f02b3d33d",
    MonsterGirls(Theme, "Monster Girls") => "dd1f77c5-dea9-4e2b-97ae-224af09caf99",
    Music(Theme, "Music") => "f42fbf9e-188a-447b-9fdc-f19dc1e4d685",
    Ninja(Theme, "Ninja") => "489dd859-9b61-4c37-af75-5b18e88daafc",
    OfficeWorkers(Theme, "Office Workers") => "92d6d951-ca5e-429c-ac78-451071cbf064",
    Police(Theme, "Police") => "df33b754-73a3-4c54-80e6-1a74a8058539",
    PostApocalyptic(Theme, "Post-Apocalyptic") => "9467335a-1b83-4497-9231-765337a00b96",
    Reincarnation(Theme, "Reincarnation") => "0bc90acb-ccc1-44ca-a34a-b9f3a73259d0",
    ReverseHarem(Theme, "Reverse Harem") => "65761a2a-415e-47f3-bef2-a9dababba7a6",
    Samurai(Theme, "Samurai") => "81183756-1453-4c81-aa9e-f6e1b63be016",
    SchoolLife(Theme, "School Life") => "caaa44eb-cd40-4177-b930-79d3ef2afe87",
    Survival(Theme, "Survival") => "5fff9cde-849c-4d78-aab0-0d52b2ee1d25",
    VideoGames(Theme, "Video Games") => "9438db5a-7e2a-4ac0-b39e-e0d95a34b8a8",
    Shota(Theme, "Shota") => "ddefd648-5140-4e5f-ba18-4eca4071d19b",
    Supernatural(Theme, "Supernatural") => "eabc5b4c-6aff-42f3-b657-3e90cbd00b75",
    TimeTravel(Theme, "Time Travel") => "292e862b-2d17-4062-90a2-0356caa4ae27",
    TraditionalGames(Theme, "Traditional Games") => "31932a7e-5b8e-49a6-9f12-2afa39dc544c",
    Vampires(Theme, "Vampires") => "d7d1730f-6eb0-4ba6-9437-602cac38664c",
    Villainess(Theme, "Villainess") => "d14322ac-4d6f-4e9b-afd9-629d5f4d8a41",
    VirtualReality(Theme, "Virtual Reality") => "8c86611e-fab7-4986-9dec-d1a2f44acdd5",
    Zombies(Theme, "Zombies") => "631ef465-9aba-4afb-b0fc-ea10efe274a8",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_to_string_works() -> anyhow::Result<()> {
        assert_eq!(
            Tag::SexualViolence.to_string(),
            "Sexual Violence".to_string()
        );
        assert_eq!(Tag::AwardWinning.to_string(), "Award Winning".to_string());
        assert_eq!(Tag::FourKoma.to_string(), "4-Koma".to_string());
        assert_eq!(Tag::BoysLove.to_string(), "Boys' Love".to_string());
        assert_eq!(Tag::GirlsLove.to_string(), "Girls' Love".to_string());
        assert_eq!(Tag::SciFi.to_string(), "Sci-Fi".to_string());
        assert_eq!(
            Tag::PostApocalyptic.to_string(),
            "Post-Apocalyptic".to_string()
        );

        Ok(())
    }

    #[test]
    fn tag_into_uuid_works() -> anyhow::Result<()> {
        let actual: Uuid = Tag::Action.into();
        let expected = Uuid::parse_str("391b0423-d847-456f-aff0-8b0cfc03066b")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn tag_try_from_uuid_works() -> anyhow::Result<()> {
        let actual = Tag::try_from(Uuid::parse_str("0234a31e-a729-4e28-9d6a-3f87c4966b9e")?)?;

        assert_eq!(actual, Tag::Oneshot);

        Ok(())
    }

    #[test]
    fn tag_try_from_non_matching_uuid_returns_error() -> anyhow::Result<()> {
        let actual = Tag::try_from(Uuid::nil());

        match actual {
            Ok(_) => panic!("expected Error::ParseError"),
            Err(err) => match err {
                Error::ParseError(e) => assert_eq!(&e, "unexpected tag UUID"),
                _ => panic!("unexpected error, expected Error::ParseError"),
            },
        }

        Ok(())
    }

    #[test]
    fn tag_group_from_tag_works() -> anyhow::Result<()> {
        assert_eq!(TagGroup::from(Tag::Gore), TagGroup::Content);
        assert_eq!(TagGroup::from(Tag::Doujinshi), TagGroup::Format);
        assert_eq!(TagGroup::from(Tag::SliceOfLife), TagGroup::Genre);
        assert_eq!(TagGroup::from(Tag::Magic), TagGroup::Theme);

        Ok(())
    }
}
