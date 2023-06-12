use std::time::Duration;

use bytes::Bytes;
use mangadex_api_schema::v5::AtHomeServer;
use mangadex_api_types::error::{Error, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use time::Date;
use url::Url;
use uuid::Uuid;

use crate::utils::get_reqwest_client;
use crate::{HttpClientRef, MangaDexClient};

pub type DownloadElement = (String, Bytes);

#[derive(Clone)]
pub enum DownloadMode {
    Normal,
    DataSaver,
}

impl Into<String> for DownloadMode{
    fn into(self) -> String {
        match self {
            Self::Normal => "data",
            Self::DataSaver => "data-saver"
        }.to_string()
    }
}

impl Default for DownloadMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Serialize, Clone)]
pub struct AtHomeReport{
    url : Url,
    success : bool,
    cached : bool,
    bytes : usize,
    duration : usize
}

pub async fn send_at_report(client : &Client, report : AtHomeReport) -> Result<Response>{
    match client.post("https://api.mangadex.network/report").json(&report).send().await {
        Ok(d) => Result::Ok(d),
        Err(e) => Result::Err(Error::RequestError(e))
    }
}

pub async fn download_chapter_image(
    http_client : &Client,
    filename : String,
    quality: DownloadMode,
    at_home: AtHomeServer,
    report : bool
) -> Result<DownloadElement> {
    let page_url = match at_home
            .base_url
            .join(&format!(
                "/{quality_mode}/{chapter_hash}/{page_filename}",
                quality_mode = Into::<String>::into(quality),
                chapter_hash = at_home.chapter.hash,
                page_filename = filename
            )){
                Ok(d) => d,
                Err(e) => return Result::Err(Error::ParseUrlError(e))
            };
    let start = tokio::time::Instant::now();
    let res : Response = match http_client.get(page_url).send().await {
        Ok(d) => d,
        Err(e) => return Err(Error::RequestError(e))
    };

    let is_cache : bool = match res.headers().get("X-Cache"){
        None => false,
        Some(d) => {
            match d.to_str() {
                Ok(val) => val.starts_with("HIT"),
                Err(_) => false
            }
        }
    };

    let bytes : Bytes = match res.bytes().await {
        Ok(d) => {
            let end = tokio::time::Instant::now();
            send_at_report(client, AtHomeReport { url: page_url, success: true, cached: is_cache, bytes: d.len(), duration: Duration::new(secs, nanos) })
        } 
    }
}

pub async fn download_chapter(
    client: &MangaDexClient,
    chapter_id: Uuid,
    quality: DownloadMode,
    report : bool
) -> Result<DownloadElement> {
    let at_home: AtHomeServer = match client.at_home().server().chapter_id(&chapter_id).build() {
        Ok(d) => d,
        Err(d) => return Result::Err(Error::RequestBuilderError(d.to_string())),
    }
    .send()
    .await?;
    let http_client = get_reqwest_client(client).await;
    let page_filenames = at_home.chapter.data;
    for filename in page_filenames {
        // If using the data-saver option, use "/data-saver/" instead of "/data/" in the URL.
        let page_url = at_home
            .base_url
            .join(&format!(
                "/{quality_mode}/{chapter_hash}/{page_filename}",
                quality_mode = "data",
                chapter_hash = at_home.chapter.hash,
                page_filename = filename
            ))
            .unwrap();

        let res = http_client.get(page_url).send().await?;
        // The data should be streamed rather than downloading the data all at once.
        let bytes: Bytes = res.bytes().await;

        // This is where you would download the file but for this example,
        // we're just printing the raw data.
        // let mut file = File::create(&filename)?;
        // let _ = file.write_all(&bytes);
        println!("Chunk: {:?}", bytes);
    }
    todo!()
}
