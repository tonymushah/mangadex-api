use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserHistoryResponse;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    default,
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct GetUserHistory {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,
}

endpoint! {
    GET "/user/history",
    #[query auth] GetUserHistory,
    #[flatten_result] UserHistoryResponse
}

#[cfg(test)]
mod tests {
    use mangadex_api_schema::v5::user_history::UserHistory;
    use mangadex_api_types::MangaDexDateTime;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_user_history_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);
        let chapid = Uuid::new_v4();
        let date: MangaDexDateTime = MangaDexDateTime::default();
        let response_body = json!({
            "result": "ok",
            "ratings": [
                {
                    "chapterId": chapid,
                    "readDate": date
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/history"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res: UserHistory = mangadex_client
            .user()
            .history()
            .get()
            .build()?
            .send()
            .await?;
        let rating = res.ratings.first().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Entry 0 not found",
        ))?;
        assert_eq!(rating.chapter_id, chapid);
        assert_eq!(rating.read_date, date);

        Ok(())
    }
}
