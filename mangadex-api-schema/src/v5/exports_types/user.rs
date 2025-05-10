use super::{ApiData, ApiObject, Results, UserAttributes};

pub type UserObject = ApiObject<UserAttributes>;
pub type UserData = ApiData<UserObject>;

pub type UserCollection = Results<UserObject>;
