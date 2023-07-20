/// Chapter Download Mode
/// Normal = "data"
/// DataSaver = "data-saver"
#[derive(Clone)]
pub enum DownloadMode {
    Normal,
    DataSaver,
}

impl From<DownloadMode> for String {
    fn from(val: DownloadMode) -> Self {
        match val {
            DownloadMode::Normal => "data",
            DownloadMode::DataSaver => "data-saver",
        }
        .to_string()
    }
}

impl Default for DownloadMode {
    fn default() -> Self {
        Self::Normal
    }
}
