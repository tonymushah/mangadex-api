use std::sync::Arc;

use bytes::Bytes;
use bytes::BytesMut;
use mangadex_api_schema::v5::AtHomeServer;
use mangadex_api_types::error::{Error, Result};
use reqwest::{Client, Response};
use tokio::pin;
use tokio::time::Instant;
use tokio_stream::StreamExt;
use url::Url;

use super::AtHomeReport;
use super::DownloadMode;

use super::DownloadElement;

#[derive(Clone)]
pub struct AtHomePreDownloadImageData {
    pub http_client: Arc<Client>,
    pub filename: String,
    pub quality: DownloadMode,
    pub at_home: Arc<AtHomeServer>,
    pub report: bool,
}

impl AtHomePreDownloadImageData {
    async fn report(
        &self,
        start: Instant,
        page_url: Url,
        bytes: usize,
        success: bool,
        cached: bool,
    ) {
        if self.report {
            let end = Instant::now();
            let _ = AtHomeReport {
                url: page_url,
                success,
                cached,
                bytes,
                duration: end.duration_since(start).as_millis(),
            }
            .send(self.http_client.clone())
            .await;
        }
    }
    pub fn build_page_url(&self) -> Result<Url> {
        match self.at_home.base_url.join(&format!(
            "/{quality_mode}/{chapter_hash}/{page_filename}",
            quality_mode = Into::<String>::into(self.quality.clone()),
            chapter_hash = self.at_home.chapter.hash,
            page_filename = self.filename
        )) {
            Ok(d) => Ok(d),
            Err(e) => Result::Err(Error::ParseUrlError(e)),
        }
    }
    pub async fn download(&self) -> DownloadElement {
        self.download_with_checker(|_, _| false).await
    }
    pub async fn download_with_checker<C>(&self, mut should_skip: C) -> DownloadElement
    where
        C: FnMut(&Self, &Response) -> bool,
    {
        let page_url = match self.build_page_url() {
            Ok(o) => o,
            Err(e) => return (self.filename.clone(), Err(e)),
        };
        let page_url_clone = page_url.clone();
        let start = tokio::time::Instant::now();
        let res: Response = match self.http_client.get(page_url).send().await {
            Ok(d) => d,
            Err(e) => {
                self.report(start, page_url_clone, 0, false, false).await;
                return (self.filename.clone(), Err(Error::RequestError(e)));
            }
        };
        if should_skip(self, &res) {
            return (
                self.filename.clone(),
                Err(Error::SkippedDownload(self.filename.clone())),
            );
        }
        let is_cache: bool = match res.headers().get("X-Cache") {
            None => false,
            Some(d) => match d.to_str() {
                Ok(val) => val.starts_with("HIT"),
                Err(_) => false,
            },
        };

        let mut bytes: BytesMut = BytesMut::new();
        let byte_stream = res.bytes_stream();
        pin!(byte_stream);
        while let Some(chunk) = byte_stream.next().await {
            match chunk {
                Ok(chunk_bytes) => {
                    bytes.extend(chunk_bytes);
                }
                Err(chunk_error) => {
                    self.report(start, page_url_clone, bytes.len(), false, is_cache)
                        .await;
                    return (self.filename.clone(), Err(Error::RequestError(chunk_error)));
                }
            }
        }
        self.report(start, page_url_clone, bytes.len(), true, is_cache)
            .await;
        (self.filename.clone(), Ok(Bytes::from(bytes)))
    }
}
