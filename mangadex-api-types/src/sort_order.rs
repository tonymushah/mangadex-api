use serde::{Deserialize, Serialize};

use crate::OrderDirection;

macro_rules! sort_order {
    (
        $(
            $( #[$meta:meta] )*
            $Enum:ident {
                $($variant:ident,)*
            }
        )?
    ) => {
        $(
            $( #[$meta] )*
            #[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
            #[serde(rename_all = "camelCase")]
            #[non_exhaustive]
            #[cfg_attr(feature = "specta", derive(specta::Type))]
            #[cfg_attr(feature = "async-graphql", derive(async_graphql::OneofObject))]
            pub enum $Enum {
                $(
                    $variant(OrderDirection),
                )*
            }
        )?
    };
}

sort_order! {
    AuthorSortOrder {
        Name,
    }
}

sort_order! {
    ChapterSortOrder {
        Chapter,
        CreatedAt,
        PublishAt,
        ReadableAt,
        UpdatedAt,
        Volume,
    }
}

sort_order! {
    CoverSortOrder {
        CreatedAt,
        UpdatedAt,
        Volume,
    }
}

sort_order! {
    GroupSortOrder {
        CreatedAt,
        FollowedCount,
        LatestUploadedChapter,
        Name,
        Relevance,
        UpdatedAt,
    }
}

sort_order! {
    MangaSortOrder {
        CreatedAt,
        FollowedCount,
        LatestUploadedChapter,
        Relevance,
        Title,
        UpdatedAt,
        Year,
        Rating,
    }
}

sort_order! {
    MangaDraftsSortOrder {
        CreatedAt,
        Title,
        UpdatedAt,
        Year,
    }
}

sort_order! {
    MangaFeedSortOrder {
        Chapter,
        CreatedAt,
        PublishAt,
        ReadableAt,
        UpdatedAt,
        Volume,
    }
}

sort_order! {
    ReportSortOrder {
        CreatedAt,
    }
}

sort_order! {
    UserSortOrder {
        Username,
    }
}
