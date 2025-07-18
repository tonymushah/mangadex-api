//! Builder for the manga read markers endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-chapter-readmarkers-2>
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
//!     .read()
//!     .get()
//!     .add_manga_id(&manga_id)
//!     .grouped(false)
//!     .send()
//!     .await?;
//!
//! println!("read markers: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::MangaReadMarkers;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
pub struct GetReadChapters {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[builder(setter(each = "add_manga_id"))]
    #[serde(rename = "ids")]
    pub manga_ids: Vec<Uuid>,

    /// Group results by manga IDs.
    ///
    /// `false` (default) will return [`MangaReadMarkers::Ungrouped`](crate::v5::schema::MangaReadMarkers::Ungrouped).
    ///
    /// `true` will return [`MangaReadMarkers::Grouped`](crate::v5::schema::MangaReadMarkers::Grouped).
    #[builder(default)]
    pub grouped: bool,
}

endpoint! {
    GET "/manga/read",
    #[query auth] GetReadChapters,
    #[flatten_result] crate::Result<MangaReadMarkers>,
    GetReadChaptersBuilder
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
    use mangadex_api_schema::v5::MangaReadMarkers;

    #[tokio::test]
    async fn get_read_markers_fires_a_request_to_base_url_ungrouped() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
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
            .and(path_regex(r"/manga/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .read()
            .get()
            .add_manga_id(manga_id)
            .send()
            .await?;

        if let MangaReadMarkers::Grouped(_) = res {
            panic!("unexpected grouped response");
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_read_markers_fires_a_request_to_base_url_grouped() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
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
            "data": {
                manga_id.to_string(): chapter_ids
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .read()
            .get()
            .add_manga_id(manga_id)
            .grouped(true)
            .send()
            .await?;

        if let MangaReadMarkers::Ungrouped(_) = res {
            panic!("unexpected ungrouped response");
        }

        Ok(())
    }
}
