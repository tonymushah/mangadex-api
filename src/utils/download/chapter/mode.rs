/// Chapter Download Mode
/// Normal = "data"
/// DataSaver = "data-saver"
#[derive(Clone)]
pub enum DownloadMode {
    Normal,
    DataSaver,
}

impl Into<String> for DownloadMode {
    fn into(self) -> String {
        match self {
            Self::Normal => "data",
            Self::DataSaver => "data-saver",
        }
        .to_string()
    }
}

impl Default for DownloadMode {
    fn default() -> Self {
        Self::Normal
    }
}
