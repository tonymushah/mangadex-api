use crate::{ApiData, ApiObject, ApiObjectNoRelationships};
pub use super::at_home_server::AtHomeServer;
pub use super::auth_tokens::AuthTokens;
pub use super::author::AuthorAttributes;
pub use super::chapter::ChapterAttributes;
pub use super::check_token_response::CheckTokenResponse;
pub use super::check_username_available::CheckUsernameAvailableResponse;
pub use super::cover::CoverAttributes;
pub use super::custom_list::CustomListAttributes;
pub use super::is_following_response::IsFollowingResponse;
pub use super::legacy_id_mapping::LegacyMappingIdAttributes;
pub use super::login_response::LoginResponse;
pub use super::manga::MangaAttributes;
pub use super::manga_aggregate::MangaAggregate;
pub use super::manga_links::MangaLinks;
pub use super::manga_read_markers::{MangaReadMarkers, UngroupedMangaReadMarkers};
pub use super::manga_reading_status::MangaReadingStatus;
pub use super::manga_reading_statuses::MangaReadingStatuses;
pub use super::manga_relation::MangaRelationAttributes;
pub use super::ratings::RatingsList;
pub use super::refresh_token_response::RefreshTokenResponse;
pub use super::report::ReportReasonAttributes;
pub use super::scanlation_group::ScanlationGroupAttributes;
pub use super::statistics::manga::MangaStatisticsObject;
pub use super::tag::TagAttributes;
pub use super::types::error::schema::MangaDexErrorResponse;
use super::types::error::Result;
pub use super::upload_session::UploadSessionResponse;
pub use super::upload_session_file::{UploadSessionFileAttributes, UploadSessionFileData};
pub use super::user::UserAttributes;
pub use super::user_report::UserReportAttributes;
pub use super::user_settings::UserSettingsAttributes;

use super::statistics::chapter::ChapterStatisticsObject;
use super::statistics::groups::GroupStatisticsObject;
use crate::v5::Results;

mod at_home;
pub use at_home::*;

mod author;
pub use author::*;

mod chapter;
pub use chapter::*;

mod cover;
pub use cover::*;

mod custom_list;
pub use custom_list::*;

mod group;
pub use group::*;

mod id_mapping;
pub use id_mapping::*;

mod manga;
pub use manga::*;

mod rating;
pub use rating::*;

mod report;
pub use report::*;

mod tag;
pub use tag::*;

mod upload;
pub use upload::*;

mod user;
pub use user::*;

mod user_reports;
pub use user_reports::*;

mod user_settings;
pub use user_settings::*;