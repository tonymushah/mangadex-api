//! Builder for the manga read markers endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-manga-chapter-readmarkers>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .get_manga_read_chapters()
//!     .manga_id(&manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("manga read markers: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::UngroupedMangaReadMarkersResponse;

#[derive(Debug, Deserialize, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct GetMangaReadChapters<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: &'a Uuid,
}

endpoint! {
    GET ("/manga/{}/read", manga_id),
    #[no_data auth] GetMangaReadChapters<'_>,
    #[flatten_result] UngroupedMangaReadMarkersResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_manga_read_markers_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let chapter_ids = [
            Uuid::parse_str("00057883-357b-4734-9469-52967e59ef4c")?,
            Uuid::parse_str("000b7978-d9bd-49ec-a8f6-a0737368415f")?,
            Uuid::parse_str("0015b621-a175-47f5-81fb-5976c88e18c4")?,
        ];
        let response_body = json!({
            "result": "ok",
            "data": chapter_ids
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .manga()
            .get_manga_read_chapters()
            .manga_id(&manga_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
