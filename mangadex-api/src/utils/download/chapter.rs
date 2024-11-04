mod mode;
mod pre_download;
mod report;
use std::sync::Arc;

use async_stream::stream;
use derive_builder::Builder;
use mangadex_api_schema::v5::AtHomeServer;
use mangadex_api_types::error::Result;
use reqwest::Response;
use tokio::pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::utils::get_reqwest_client;
use crate::{HttpClientRef, MangaDexClient};

use super::DownloadElement;

pub use mode::DownloadMode;
pub use pre_download::AtHomePreDownloadImageData;
pub use report::AtHomeReport;

#[derive(Clone, Builder)]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
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
        let http_client = Arc::new(get_reqwest_client(&client).await);
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
    use std::{
        fs::{create_dir_all, File},
        io::Write,
    };
    use tokio::pin;
    use tokio_stream::StreamExt;

    /// It's from this manga called [`The Grim Reaper Falls In Love With A Human`](https://mangadex.org/title/be2efc56-1669-4e42-9f27-3bd232bca8ea/the-grim-reaper-falls-in-love-with-a-human)
    ///
    /// [Chapter 1 English](https://mangadex.org/chapter/2b4e39a5-fba0-4055-a176-8b7e19faacdb) by [`Kredim`](https://mangadex.org/group/0b870e54-c75f-4d2e-8068-c40f939135fd/kredim)
    #[tokio::test]
    async fn download_chapter_save() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("32b229f6-e9bf-41a0-9694-63c11191704c")?;
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
                file.write_all(&bytes)?
            };
        }
        Ok(())
    }

    /// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
    ///
    /// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
    #[tokio::test]
    async fn download_chapter_with_streams() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
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
                file.write_all(&bytes)?
            }
        }
        Ok(())
    }

    /// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
    ///
    /// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
    #[tokio::test]
    async fn download_chapter_with_streams_and_checker() -> Result<()> {
        let output_dir = "./test-outputs/";
        let client = MangaDexClient::default();
        let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
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
                file.write_all(&bytes)?;
                println!("Downloaded {filename}");
            } else {
                println!("Skipped {filename}");
            }
        }
        Ok(())
    }
}
