#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::at_home::server::id::get::GetAtHomeServerBuilder;

use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct AtHomeServerParams {
    chapter_id: Uuid,
    #[serde(default)]
    force_port_443: Option<bool>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AtHomeServerParams> for GetAtHomeServerBuilder {
    fn from(value: AtHomeServerParams) -> Self {
        let mut builder = Self::default();
        builder.chapter_id(value.chapter_id);
        builder.force_port_443(value.force_port_443.unwrap_or_default());
        builder
    }
}
