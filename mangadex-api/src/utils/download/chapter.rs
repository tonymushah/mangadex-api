mod mode;
mod pre_download;
mod report;
use std::sync::Arc;

use crate::Result;
use async_stream::stream;
use derive_builder::Builder;
use mangadex_api_schema::v5::AtHomeServer;
use reqwest::Response;
use tokio::pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{HttpClientRef, MangaDexClient};

use super::DownloadElement;

pub use mode::DownloadMode;
pub use pre_download::AtHomePreDownloadImageData;
pub use report::AtHomeReport;

#[derive(Clone, Builder)]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct ChapterDownload {
    #[doc(hidden)]
    #[builder(pattern = "immutable")]
    http_client: HttpClientRef,
    /// Download mode
    mode: Option<DownloadMode>,
    /// Enable reporting at `api.mangadex.network`. \
    /// More details at : https://api.mangadex.org/docs/retrieving-chapter/#mangadexhome-load-successes-failures-and-retries
    report: Option<bool>,
    /// Force selecting from MangaDex@Home servers that use the standard HTTPS port 443.
    ///
    /// While the conventional port for HTTPS traffic is 443 and servers are encouraged to use it,
    /// it is not a hard requirement as it technically isn't anything special.
    ///
    /// However, some misbehaving school/office network will at time block traffic to non-standard
    /// ports, and setting this flag to true will ensure selection of a server that uses these.
    force_port_443: bool,
    /// Chapter Id
    id: Uuid,
}

impl ChapterDownload {
    pub async fn build_at_home_urls_as_stream(
        &self,
    ) -> Result<impl Stream<Item = AtHomePreDownloadImageData> + '_> {
        let client = MangaDexClient::new_with_http_client_ref(self.http_client.clone());
        let at_home: Arc<AtHomeServer> = Arc::new(
            client
                .at_home()
                .server()
                .id(self.id)
                .get()
                .force_port_443(self.force_port_443)
                .send()
                .await?
                .body,
        );
        let http_client = client.get_reqwest_client().await;
        let page_filenames = match self.mode.unwrap_or_default() {
            DownloadMode::Normal => Arc::clone(&at_home).chapter.data.clone(),
            DownloadMode::DataSaver => Arc::clone(&at_home).chapter.data_saver.clone(),
        };

        Ok(stream! {
            for filename in page_filenames {
                yield AtHomePreDownloadImageData {
                    http_client: http_client.clone(),
                    filename: filename.clone(),
                    quality: self.mode.unwrap_or_default(),
                    at_home: Arc::clone(&at_home),
                    report: self.report.unwrap_or(false),
                };
            }
        })
    }
    pub async fn build_at_home_urls(&self) -> Result<Vec<AtHomePreDownloadImageData>> {
        let mut datas: Vec<AtHomePreDownloadImageData> = Vec::new();
        let stream_ = self.build_at_home_urls_as_stream().await?;
        pin!(stream_);
        while let Some(data) = stream_.next().await {
            datas.push(data);
        }
        Ok(datas)
    }
    pub async fn download_element_vec(&self) -> Result<Vec<DownloadElement>> {
        let file_names = self.build_at_home_urls().await?;
        let mut datas: Vec<DownloadElement> = Vec::new();
        for filename in file_names {
            datas.push(filename.download().await);
        }
        Ok(datas)
    }
    pub async fn download_stream(
        &self,
    ) -> Result<impl Stream<Item = (DownloadElement, usize, usize)> + '_> {
        let file_names = self.build_at_home_urls().await?;
        let mut index: usize = 0;
        let len = file_names.len();
        Ok(stream! {
            for filename in file_names {
                let data = filename.download().await;
                index += 1;
                yield (data, index, len);
            }
        })
    }
    /// Download chapter with stream output
    pub async fn download_stream_with_checker<C>(
        &self,
        should_check_: C,
    ) -> Result<impl Stream<Item = (DownloadElement, usize, usize)>>
    where
        C: FnMut(&AtHomePreDownloadImageData, &Response) -> bool + std::marker::Copy,
    {
        let file_names = self.build_at_home_urls().await?;
        let mut index: usize = 0;
        let len = file_names.len();
        Ok(stream! {
            for filename in file_names {
                let data = filename.download_with_checker(should_check_).await;
                index += 1;
                yield (data, index, len);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::download::chapter::DownloadMode, MangaDexClient};
    use anyhow::Result;
    use bytes::Buf;
    use std::{
        fs::{create_dir_all, File},
        io::{BufWriter, Write},
    };
    use tokio::pin;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn download_chapter_save() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("e716db76-fefa-46c1-8b0a-3a7a8879d7d2")?;
        let chapter_files = client
            .download()
            .chapter(chapter_id)
            .mode(DownloadMode::DataSaver)
            .report(true)
            .build()?
            .download_element_vec()
            .await?;
        create_dir_all(format!("{}{}", output_dir, chapter_id))?;
        for (filename, bytes_) in chapter_files {
            if let Ok(bytes) = bytes_ {
                let mut file: File =
                    File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
                {
                    let mut buffer = BufWriter::new(&mut file);
                    std::io::copy(&mut bytes.reader(), &mut buffer)?;
                    buffer.flush()?;
                }
            };
        }
        Ok(())
    }

    #[tokio::test]
    async fn download_chapter_with_streams() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("2be303df-2853-490c-93e0-d4544634024e")?;
        create_dir_all(format!("{}{}", output_dir, chapter_id))?;
        let download = client
            .download()
            .chapter(chapter_id)
            .mode(DownloadMode::DataSaver)
            .report(true)
            .build()?;
        let chapter_files = download.download_stream().await?;
        pin!(chapter_files);
        while let Some((data, _, _)) = chapter_files.next().await {
            let (filename, bytes_) = data;
            if let Ok(bytes) = bytes_ {
                let mut file: File =
                    File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
                {
                    let mut buffer = BufWriter::new(&mut file);
                    std::io::copy(&mut bytes.reader(), &mut buffer)?;
                    buffer.flush()?;
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn download_chapter_with_streams_and_checker() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("0c364253-a720-44ff-8e7b-9119c64da767")?;
        create_dir_all(format!("{}{}", output_dir, chapter_id))?;
        let download = client
            .download()
            .chapter(chapter_id)
            .mode(DownloadMode::DataSaver)
            .report(true)
            .build()?;
        let chapter_files = download
            .download_stream_with_checker(move |filename, response| {
                let is_skip: bool = {
                    let content_length = match response.content_length() {
                        None => return false,
                        Some(d) => d,
                    };
                    if let core::result::Result::Ok(pre_file) = File::open(format!(
                        "{}{}/{}",
                        output_dir,
                        chapter_id,
                        filename.filename.clone()
                    )) {
                        if let core::result::Result::Ok(metadata) = pre_file.metadata() {
                            metadata.len() == content_length
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                is_skip
            })
            .await?;
        pin!(chapter_files);
        while let Some((data, index, len)) = chapter_files.next().await {
            print!("{index} - {len} : ");
            let (filename, bytes_) = data;
            if let Ok(bytes) = bytes_ {
                let mut file: File =
                    File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
                {
                    let mut buffer = BufWriter::new(&mut file);
                    std::io::copy(&mut bytes.reader(), &mut buffer)?;
                    buffer.flush()?;
                }
                println!("Downloaded {filename}");
            } else {
                println!("Skipped {filename}");
            }
        }
        Ok(())
    }
}
