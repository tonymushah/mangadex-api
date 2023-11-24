use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::post::{UploadImage, UploadImagesBuilder},
    MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::UploadSessionFileDataObject, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct UploadImageParam {
    pub session_id: Uuid,
    pub files: Vec<PathBuf>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UploadImageParam> for UploadImagesBuilder {
    fn from(value: UploadImageParam) -> Self {
        let mut builder = Self::default();
        builder.session_id(value.session_id);
        let files: Vec<UploadImage> = value
            .files
            .iter()
            .filter(|f| {
                f.is_file()
                    && f.extension()
                        .is_some_and(|e| ["jpg", "jpeg", "png", "gif"].iter().any(|a| *e == **a))
            })
            .flat_map(|f| {
                let res: std::io::Result<UploadImage> = {
                    let filename = f
                        .to_str()
                        .ok_or(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            String::from("Can't find the filename"),
                        ))?
                        .to_string();
                    let mut data = Vec::<u8>::new();
                    let mut buf_reader = BufReader::new(File::open(f)?);
                    buf_reader.read_to_end(&mut data)?;
                    Ok(UploadImage { filename, data })
                };
                res
            })
            .collect();
        builder.files(files);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UploadImageParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> Result<Limited<UploadSessionFileDataObject>> {
        <UploadImagesBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
