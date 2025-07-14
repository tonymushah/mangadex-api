use serde::{Deserialize, Serialize};

use crate::{ReferenceExpansionResource, RelationshipType};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum ForumThreadType {
    Manga,
    Group,
    Chapter,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ForumThreadTypeParseError {
    #[error("the input {0} is invalid")]
    InvalidInput(String),
}

impl TryFrom<RelationshipType> for ForumThreadType {
    type Error = ForumThreadTypeParseError;
    fn try_from(value: RelationshipType) -> Result<Self, Self::Error> {
        match value {
            RelationshipType::Manga => Ok(Self::Manga),
            RelationshipType::Chapter => Ok(Self::Chapter),
            RelationshipType::ScanlationGroup => Ok(Self::Group),
            i => Err(ForumThreadTypeParseError::InvalidInput(format!("{i:?}"))),
        }
    }
}

impl TryFrom<ReferenceExpansionResource> for ForumThreadType {
    type Error = ForumThreadTypeParseError;
    fn try_from(value: ReferenceExpansionResource) -> Result<Self, Self::Error> {
        match value {
            ReferenceExpansionResource::Manga => Ok(Self::Manga),
            ReferenceExpansionResource::Chapter => Ok(Self::Chapter),
            ReferenceExpansionResource::ScanlationGroup => Ok(Self::Group),
            i => Err(ForumThreadTypeParseError::InvalidInput(format!("{i:?}"))),
        }
    }
}
