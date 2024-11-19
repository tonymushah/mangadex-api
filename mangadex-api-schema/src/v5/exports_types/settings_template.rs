use crate::v5::settings_template::UserSettingsTemplateAttributes;

use super::{ApiData, ApiObject, Result, Results};

pub type UserSettingsTemplateObject = ApiObject<UserSettingsTemplateAttributes>;
pub type UserSettingsTemplateData = ApiData<UserSettingsTemplateObject>;
pub type UserSettingsTemplateResponse = Result<UserSettingsTemplateData>;
pub type UserSettingsTemplateCollection = Results<UserSettingsTemplateObject>;
pub type UserSettingsTemplateListResponse = Result<UserSettingsTemplateCollection>;
