use crate::{error::Error, HttpClientRef, MangaDexClient, Result, CDN_URL};
use derive_builder::Builder;
use mangadex_api_schema::{
    v5::{CoverAttributes, MangaAttributes, RelatedAttributes},
    ApiObject,
};
use mangadex_api_types::{
    CoverSortOrder, OrderDirection, ReferenceExpansionResource, RelationshipType,
};
use reqwest::Client;
use url::Url;
use uuid::Uuid;

use super::DownloadElement;

#[derive(Clone, Copy)]
pub enum CoverQuality {
    Default = 0,
    /// For 512 cover fetching
    Size512 = 512,
    /// For 256 cover fetching
    Size256 = 256,
}

impl Default for CoverQuality {
    fn default() -> Self {
        Self::Default
    }
}

/// Download a Mangadex Manga Cover Image vie :
/// - The filename
/// - The manga_id
pub async fn download_cover(
    client: &Client,
    file_name: String,
    manga_id: Uuid,
    cover_quality: CoverQuality,
) -> DownloadElement {
    let file_name = match cover_quality {
        CoverQuality::Default => file_name,
        CoverQuality::Size256 => {
            format!("{}.{}.jpg", file_name, 256)
        }
        CoverQuality::Size512 => {
            format!("{}.{}.jpg", file_name, 512)
        }
    };
    let cover_url = match Url::parse(&format!("{}/covers/{}/{}", CDN_URL, manga_id, file_name)) {
        Ok(d) => d,
        Err(e) => return (file_name, Err(Error::ParseError(e.to_string()))),
    };
    let res = match client.get(cover_url).send().await {
        Err(e) => return (file_name, Err(Error::RequestError(e))),
        Ok(d) => d,
    };
    let bytes = match res.bytes().await {
        Err(e) => return (file_name, Err(Error::RequestError(e))),
        Ok(d) => d,
    };
    (file_name, Ok(bytes))
}

pub async fn download_via_cover_api_object(
    http_client: HttpClientRef,
    cover: ApiObject<CoverAttributes>,
    cover_quality: CoverQuality,
) -> DownloadElement {
    let mangadex_api_client = MangaDexClient::new_with_http_client_ref(http_client);
    let file_name = cover.attributes.file_name;
    // Check if the manga id available in the relationship
    let manga_id = match cover
        .relationships
        .iter()
        .find(|relationship| relationship.type_ == RelationshipType::Manga)
    {
        Some(manga) => manga.id,
        None => {
            return (
                file_name,
                Err(Error::unknow(format!(
                    "Manga relationship not found in cover {} object",
                    cover.id
                ))),
            )
        }
    };
    let client = mangadex_api_client.get_reqwest_client().await;
    download_cover(&client, file_name, manga_id, cover_quality).await
}

pub async fn download_via_cover_id(
    http_client: HttpClientRef,
    cover_id: Uuid,
    cover_quality: CoverQuality,
) -> Result<DownloadElement> {
    let mangadex_api_client = MangaDexClient::new_with_http_client_ref(http_client.clone());
    let cover = match mangadex_api_client.cover().cover_id(cover_id).get().build() {
        Ok(d) => d,
        Err(e) => return Err(Error::RequestBuilderError(e.to_string())),
    }
    .send()
    .await?;
    Ok(download_via_cover_api_object(http_client, cover.data, cover_quality).await)
}

pub async fn download_via_manga_api_object(
    http_client: HttpClientRef,
    manga: ApiObject<MangaAttributes>,
    cover_quality: CoverQuality,
) -> Result<DownloadElement> {
    let mangadex_api_client = MangaDexClient::new_with_http_client_ref(http_client.clone());
    // Search if there is a cover relationship object in the MangaObject
    let file_name: String = match manga
        .relationships
        .iter()
        .find(|relationship| relationship.type_ == RelationshipType::CoverArt)
    {
        // Finds the relationship
        Some(relationship) => {
            // Verify if this relationship has attributes
            if let Some(relationship_attr) = &relationship.attributes {
                match relationship_attr {
                    // Get the filename
                    RelatedAttributes::CoverArt(cover) => cover.file_name.clone(),
                    // Getting it via the `MangadexClient` otherwise
                    _ => {
                        match mangadex_api_client
                            .cover()
                            .cover_id(relationship.id)
                            .get()
                            .build()
                        {
                            Ok(d) => d,
                            Err(e) => return Err(Error::RequestBuilderError(e.to_string())),
                        }
                        .send()
                        .await?
                        .data
                        .attributes
                        .file_name
                    }
                }
            }
            // Getting it via the `MangadexClient` otherwise
            else {
                mangadex_api_client
                    .cover()
                    .cover_id(relationship.id)
                    .get()
                    .build()?
                    .send()
                    .await?
                    .data
                    .attributes
                    .file_name
            }
        }
        // Getting the file name via the list of the manga cover ordered by volume `desc` otherwise
        None => {
            match mangadex_api_client
                .cover()
                .get()
                .add_manga_id(&manga.id)
                .order(CoverSortOrder::Volume(OrderDirection::Descending))
                .build()
            {
                Ok(d) => match d.send().await?.data.first() {
                    None => return Err(Error::unknow("can't find the first cover of this manga")),
                    Some(cover) => cover.attributes.file_name.clone(),
                },
                Err(e) => return Err(Error::RequestBuilderError(e.to_string())),
            }
        }
    };
    let client = mangadex_api_client.get_reqwest_client().await;
    Ok(download_cover(&client, file_name, manga.id, cover_quality).await)
}

pub async fn download_via_manga_id(
    http_client: HttpClientRef,
    manga_id: Uuid,
    cover_quality: CoverQuality,
) -> Result<DownloadElement> {
    let mangadex_api_client = MangaDexClient::new_with_http_client_ref(http_client.clone());
    let manga: ApiObject<MangaAttributes> = match mangadex_api_client
        .manga()
        .id(manga_id)
        .get()
        .includes(vec![ReferenceExpansionResource::CoverArt])
        .build()
    {
        Ok(res) => res.send().await?.data,
        Err(e) => return Err(Error::RequestBuilderError(e.to_string())),
    };
    download_via_manga_api_object(http_client, manga, cover_quality).await
}

#[derive(Clone, Builder)]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct CoverDownload {
    #[doc(hidden)]
    #[builder(pattern = "immutable")]
    http_client: HttpClientRef,

    quality: CoverQuality,
}

impl CoverDownload {
    pub async fn via_cover_api_object(&self, cover: ApiObject<CoverAttributes>) -> DownloadElement {
        download_via_cover_api_object(self.http_client.clone(), cover, self.quality).await
    }
    pub async fn via_cover_id(&self, cover_id: Uuid) -> Result<DownloadElement> {
        download_via_cover_id(self.http_client.clone(), cover_id, self.quality).await
    }
    pub async fn via_manga_api_object(
        &self,
        manga: ApiObject<MangaAttributes>,
    ) -> Result<DownloadElement> {
        download_via_manga_api_object(self.http_client.clone(), manga, self.quality).await
    }
    pub async fn via_manga_id(&self, manga_id: Uuid) -> Result<DownloadElement> {
        download_via_manga_id(self.http_client.clone(), manga_id, self.quality).await
    }
}

#[cfg(test)]
mod tests {
    use crate::MangaDexClient;
    use anyhow::Result;
    use std::{
        fs::{create_dir_all, File},
        io::Write,
    };
    use uuid::Uuid;

    /// Download the volume 2 cover of [Lycoris Recoil](https://mangadex.org/title/9c21fbcd-e22e-4e6d-8258-7d580df9fc45/lycoris-recoil)
    #[tokio::test]
    pub async fn via_cover_id() -> Result<()> {
        let cover_id: Uuid = Uuid::parse_str("b5040c5d-7fd3-4b17-af5f-c96805c83aa4")?;
        let client: MangaDexClient = MangaDexClient::default();
        let (filename, bytes) = client
            .download()
            .cover()
            .build()?
            .via_cover_id(cover_id)
            .await?;
        create_dir_all("test-outputs/covers")?;
        let mut file = File::create(format!("{}/{}", "test-outputs/covers", filename))?;
        file.write_all(&bytes.unwrap())?;
        Ok(())
    }

    /// Download the [Kimi tte Watashi no Koto Suki Nandesho?](https://mangadex.org/title/f75c2845-0241-4e69-87c7-b93575b532dd/kimi-tte-watashi-no-koto-suki-nandesho) cover
    ///
    /// For test... of course :3
    #[tokio::test]
    pub async fn via_manga_id() -> Result<()> {
        let manga_id: Uuid = Uuid::parse_str("f75c2845-0241-4e69-87c7-b93575b532dd")?;
        let client: MangaDexClient = MangaDexClient::default();
        let (filename, bytes) = client
            .download()
            .cover()
            .build()?
            .via_manga_id(manga_id)
            .await?;
        create_dir_all("test-outputs/covers")?;
        let mut file = File::create(format!("{}/{}", "test-outputs/covers", filename))?;
        file.write_all(&bytes.unwrap())?;
        Ok(())
    }
}
