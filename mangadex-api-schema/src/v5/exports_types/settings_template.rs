use crate::v5::settings_template::UserSettingsTemplateAttributes;

use super::{ApiData, ApiObject, Results};

pub type UserSettingsTemplateObject = ApiObject<UserSettingsTemplateAttributes>;
pub type UserSettingsTemplateData = ApiData<UserSettingsTemplateObject>;
pub type UserSettingsTemplateCollection = Results<UserSettingsTemplateObject>;
