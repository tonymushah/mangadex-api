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

pub type AtHomeServerResponse = Result<AtHomeServer>;

pub type AuthorObject = ApiObject<AuthorAttributes>;
pub type AuthorData = ApiData<AuthorObject>;
pub type AuthorResponse = Result<AuthorData>;
pub type AuthorCollection = Results<AuthorObject>;
pub type AuthorListResponse = Result<AuthorCollection>;

pub type ChapterObject = ApiObject<ChapterAttributes>;
pub type ChapterData = ApiData<ChapterObject>;
pub type ChapterResponse = Result<ChapterData>;
pub type ChapterCollection = Results<ChapterObject>;
pub type ChapterListResponse = Result<ChapterCollection>;

pub type CoverObject = ApiObject<CoverAttributes>;
pub type CoverData = ApiData<CoverObject>;
pub type CoverResponse = Result<CoverData>;
pub type CoverCollection = Results<CoverObject>;
pub type CoverListResponse = Result<CoverCollection>;

pub type CustomListObject = ApiObject<CustomListAttributes>;
pub type CustomListData = ApiData<CustomListObject>;
pub type CustomListResponse = Result<CustomListData>;
pub type CustomListCollection = Results<CustomListObject>;
pub type CustomListListResponse = Result<CustomListCollection>;

pub type GroupObject = ApiObject<ScanlationGroupAttributes>;
pub type GroupData = ApiData<GroupObject>;
pub type GroupResponse = Result<GroupData>;
pub type GroupCollection = Results<GroupObject>;
pub type GroupListResponse = Result<GroupCollection>;

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

pub type ChapterStatisticsResponse = Result<ChapterStatisticsObject>;

pub type GroupStatisticsResponse = Result<GroupStatisticsObject>;

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