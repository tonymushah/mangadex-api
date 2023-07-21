/// TODO Factor and split export to multiple files

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

pub type IdMappingObject = ApiObject<LegacyMappingIdAttributes>;
pub type IdMappingData = ApiData<IdMappingObject>;
pub type IdMappindCollection = Results<IdMappingObject>;
pub type IdMappingListResponse = Result<IdMappindCollection>;

pub type MangaObject = ApiObject<MangaAttributes>;
pub type MangaData = ApiData<MangaObject>;
pub type MangaResponse = Result<MangaData>;
pub type MangaCollection = Results<MangaObject>;
pub type MangaListResponse = Result<MangaCollection>;

pub type MangaAggregateResponse = Result<MangaAggregate>;

pub type UngroupedMangaReadMarkersResponse = Result<UngroupedMangaReadMarkers>;
pub type MangaReadMarkersResponse = Result<MangaReadMarkers>;

pub type MangaReadingStatusResponse = Result<MangaReadingStatus>;
pub type MangaReadingStatusesResponse = Result<MangaReadingStatuses>;

pub type MangaRelationObject = ApiObject<MangaRelationAttributes>;
pub type MangaRelationCollection = Results<MangaRelationObject>;
pub type MangaRelationListResponse = Result<MangaRelationCollection>;

pub type MangaStatisticsResponse = Result<MangaStatisticsObject>;

pub type RatingsResponse = Result<RatingsList>;

pub type ReportReasonObject = ApiObjectNoRelationships<ReportReasonAttributes>;
pub type ReportReasonCollection = Results<ReportReasonObject>;
pub type ReportReasonListResponse = Result<ReportReasonCollection>;

pub type TagObject = ApiObject<TagAttributes>;
pub type TagData = ApiData<TagObject>;
pub type TagResponse = Result<TagData>;
pub type TagCollection = Results<TagObject>;
pub type TagListResponse = Result<TagCollection>;

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileResponse = Result<UploadSessionFileData<UploadSessionFileObject>>;

pub type UserObject = ApiObject<UserAttributes>;
pub type UserData = ApiData<UserObject>;
pub type UserResponse = Result<UserData>;
pub type UserCollection = Results<UserObject>;
pub type UserListResponse = Result<UserCollection>;

pub type UserReportsObject = ApiObject<UserReportAttributes>;
pub type UserReportsData = ApiData<UserReportsObject>;
pub type UserReportCollection = Results<UserReportsObject>;
pub type UserReportsListResponse = Result<UserReportCollection>;

pub type UserSettingsResponse = Result<UserSettingsAttributes>;