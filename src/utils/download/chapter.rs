use std::sync::Arc;

use async_stream::stream;
use bytes::Bytes;
use derive_builder::Builder;
use mangadex_api_schema::v5::AtHomeServer;
use mangadex_api_types::error::{Error, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use tokio::time::Instant;
use tokio_stream::Stream;
use url::Url;
use uuid::Uuid;

use crate::utils::get_reqwest_client;
use crate::{HttpClientRef, MangaDexClient};

use super::DownloadElement;

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

/// Send a report to `https://api.mangadex.network/report`.
///
/// More details at : https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint
#[derive(Serialize, Clone)]
pub struct AtHomeReport {
    url: Url,
    success: bool,
    cached: bool,
    bytes: usize,
    duration: u128,
}

impl AtHomeReport {
    pub async fn send(&self, client: &Client) -> Result<Response> {
        if !self.url.as_str().contains("mangadex.org") {
            match client
                .post("https://api.mangadex.network/report")
                .json(self)
                .send()
                .await
            {
                Ok(d) => Result::Ok(d),
                Err(e) => Result::Err(Error::RequestError(e)),
            }
        } else {
            Result::Err(Error::UnexpectedError(anyhow::Error::msg(
                "the mangadex.org pattern found!",
            )))
        }
    }
}

pub async fn download_chapter_image(
    http_client: &Client,
    filename: String,
    quality: DownloadMode,
    at_home: Arc<AtHomeServer>,
    report: bool,
) -> Result<DownloadElement> {
    let page_url = match at_home.base_url.join(&format!(
        "/{quality_mode}/{chapter_hash}/{page_filename}",
        quality_mode = Into::<String>::into(quality),
        chapter_hash = at_home.chapter.hash,
        page_filename = filename
    )) {
        Ok(d) => d,
        Err(e) => return Result::Err(Error::ParseUrlError(e)),
    };
    let page_url_clone = page_url.clone();
    let start = tokio::time::Instant::now();
    let res: Response = match http_client.get(page_url).send().await {
        Ok(d) => d,
        Err(e) => {
            if report {
                let end = Instant::now();
                let _ = AtHomeReport {
                    url: page_url_clone,
                    success: false,
                    cached: false,
                    bytes: 0,
                    duration: end.duration_since(start).as_millis(),
                }
                .send(http_client)
                .await;
            }
            return Err(Error::RequestError(e));
        }
    };

    let is_cache: bool = match res.headers().get("X-Cache") {
        None => false,
        Some(d) => match d.to_str() {
            Ok(val) => val.starts_with("HIT"),
            Err(_) => false,
        },
    };
    let content_length = match &res.content_length() {
        None => 0,
        Some(d) => usize::into((*d).try_into().unwrap()),
    };

    let bytes: Bytes = match res.bytes().await {
        Ok(d) => {
            let end = Instant::now();
            if report {
                let _ = AtHomeReport {
                    url: page_url_clone,
                    success: true,
                    cached: is_cache,
                    bytes: d.len(),
                    duration: end.duration_since(start).as_millis(),
                }
                .send(http_client)
                .await;
            }
            d
        }
        Err(e) => {
            let end = Instant::now();
            if report {
                let _ = AtHomeReport {
                    url: page_url_clone,
                    success: false,
                    cached: is_cache,
                    bytes: content_length,
                    duration: end.duration_since(start).as_millis(),
                }
                .send(http_client)
                .await;
            }
            return Err(Error::RequestError(e));
        }
    };

    Ok((filename, bytes))
}

/// The function behind `ChapterDownload::execute`
pub async fn download_chapter(
    client: HttpClientRef,
    chapter_id: Uuid,
    quality: DownloadMode,
    report: bool,
    force_443_port: bool,
) -> Result<Vec<DownloadElement>> {
    let client = MangaDexClient::new_with_http_client_ref(client);
    let at_home: Arc<AtHomeServer> = Arc::new(
        match client
            .at_home()
            .server()
            .force_port_443(force_443_port)
            .chapter_id(&chapter_id)
            .build()
        {
            Ok(d) => d,
            Err(d) => return Result::Err(Error::RequestBuilderError(d.to_string())),
        }
        .send()
        .await?,
    );
    let http_client = get_reqwest_client(&client).await;
    let page_filenames = match quality {
        DownloadMode::Normal => &at_home.chapter.data,
        DownloadMode::DataSaver => &at_home.chapter.data_saver,
    };
    let mut datas: Vec<DownloadElement> = Vec::new();
    for filename in page_filenames {
        datas.push(
            download_chapter_image(
                &http_client,
                filename.to_string(),
                quality.clone(),
                at_home.clone(),
                report,
            )
            .await?,
        );
    }
    Ok(datas)
}

pub async fn download_chapter_stream(
    client: HttpClientRef,
    chapter_id: Uuid,
    quality: DownloadMode,
    report: bool,
    force_443_port: bool,
) -> Result<impl Stream<Item = Result<DownloadElement>>> {
    let client = MangaDexClient::new_with_http_client_ref(client);
    let at_home: Arc<AtHomeServer> = Arc::new(
        match client
            .at_home()
            .server()
            .force_port_443(force_443_port)
            .chapter_id(&chapter_id)
            .build()
        {
            Ok(d) => d,
            Err(d) => return Result::Err(Error::RequestBuilderError(d.to_string())),
        }
        .send()
        .await?,
    );
    let http_client = get_reqwest_client(&client).await;
    let page_filenames = match quality {
        DownloadMode::Normal => Arc::clone(&at_home).chapter.data.clone(),
        DownloadMode::DataSaver => Arc::clone(&at_home).chapter.data_saver.clone(),
    };
    Ok(stream! {
        for filename in page_filenames {
            yield download_chapter_image(
                &http_client,
                filename.to_string(),
                quality.clone(),
                at_home.clone(),
                report,
            )
            .await;
        }
    })
}

#[derive(Clone, Builder)]
#[builder(setter(into, strip_option), pattern = "owned")]
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
    pub async fn get_download_element_vec(&self) -> Result<Vec<DownloadElement>> {
        download_chapter(
            self.http_client.clone(),
            self.id,
            if let Some(quality) = self.mode.clone() {
                quality
            } else {
                Default::default()
            },
            if let Some(report) = self.report {
                report
            } else {
                false
            },
            self.force_port_443,
        )
        .await
    }
    pub async fn execute(&self) -> Result<Vec<DownloadElement>> {
        self.get_download_element_vec().await
    }
    pub async fn execute_stream(&self) -> Result<impl Stream<Item = Result<DownloadElement>>> {
        download_chapter_stream(
            self.http_client.clone(),
            self.id,
            if let Some(quality) = self.mode.clone() {
                quality
            } else {
                Default::default()
            },
            if let Some(report) = self.report {
                report
            } else {
                false
            },
            self.force_port_443,
        )
        .await
    } 
}

#[cfg(test)]
mod tests {
    use crate::{utils::download::chapter::DownloadMode, MangaDexClient};
    use anyhow::{Ok, Result};
    use std::{
        fs::{create_dir_all, File},
        io::Write,
    };

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
            .execute()
            .await?;
        create_dir_all(format!("{}{}", output_dir, chapter_id))?;
        for (filename, bytes) in chapter_files {
            let mut file: File =
                File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
        }
        Ok(())
    }
}
