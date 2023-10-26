use crate::v5::user_history::UserHistory;

use super::{ApiData, ApiObject, Result, Results, UserAttributes};

pub type UserObject = ApiObject<UserAttributes>;
pub type UserData = ApiData<UserObject>;

pub type UserResponse = Result<UserData>;

pub type UserCollection = Results<UserObject>;
pub type UserListResponse = Result<UserCollection>;

pub type UserHistoryResponse = Result<UserHistory>;
