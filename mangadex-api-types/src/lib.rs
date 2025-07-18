#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]

pub mod api_client_profile;
pub mod api_client_state;
pub mod error;
pub mod forum_thread;
pub mod include_empty_pages;
pub mod include_external_url;
pub mod include_future_publish_at;
pub mod include_future_updates;
pub mod include_unavailable;
pub mod language;
pub mod legacy_mapping_type;
pub mod manga_link;
pub mod manga_state;
pub mod mangadex_datetime;
pub mod mangadex_duration;
pub mod oauth;
pub mod order_direction;
pub mod password;
pub mod report_category;
pub mod report_status;
pub mod result;
pub mod sort_order;
pub mod static_data;
pub mod tag;
pub mod tag_search_mode;
pub mod upload_source;
pub mod user_role;
pub mod username;

pub use api_client_profile::ApiClientProfile;
pub use api_client_state::ApiClientState;
pub use forum_thread::ForumThreadType;
pub use include_empty_pages::IncludeFuturePages;
pub use include_external_url::IncludeExternalUrl;
pub use include_future_publish_at::IncludeFuturePublishAt;
pub use include_future_updates::IncludeFutureUpdates;
pub use include_unavailable::IncludeUnvailable;
pub use language::Language;
pub use legacy_mapping_type::LegacyMappingType;
pub use manga_link::{MangaLink, MangaLinks};
pub use manga_state::MangaState;
pub use mangadex_datetime::MangaDexDateTime;
pub use mangadex_duration::MangaDexDuration;
pub use order_direction::OrderDirection;
pub use password::Password;
pub use report_category::ReportCategory;
pub use report_status::ReportStatus;
pub use result::ResultType;
pub use sort_order::*;
pub use static_data::content_rating::ContentRating;
pub use static_data::custom_list_visibility::CustomListVisibility;
pub use static_data::demographic::Demographic;
pub use static_data::manga_relation::MangaRelation;
pub use static_data::manga_status::MangaStatus;
pub use static_data::reading_status::ReadingStatus;
pub use static_data::reference_expansion_resource::ReferenceExpansionResource;
pub use static_data::relationship_type::RelationshipType;
pub use static_data::response_type::ResponseType;
pub use tag::{Tag, TagGroup};
pub use tag_search_mode::TagSearchMode;
pub use upload_source::UploadSource;
pub use user_role::UserRole;
pub use username::Username;

#[macro_use]
pub(crate) mod macros;
