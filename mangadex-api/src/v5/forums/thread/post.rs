use crate::HttpClientRef;
use derive_builder::Builder;
use mangadex_api_schema::v5::ForumThreadResponse;
use mangadex_api_types::ForumThreadType;
use serde::Serialize;
use uuid::Uuid;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct CreateForumThread {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub type_: ForumThreadType,
    pub id: Uuid,
}

endpoint! {
    POST "/forums/thread",
    #[body auth] CreateForumThread,
    #[flatten_result] ForumThreadResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::ForumThreadType;

    #[tokio::test]
    async fn create_a_forums_thread_handle_ok() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let body_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "type": "thread",
                "id": 0,
                "attributes": {
                    "repliesCount": 0
                }
            }
        });

        Mock::given(method("POST"))
            .and(path_regex("/forums/thread"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({
                "type": "manga",
                "id": body_id
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .forums()
            .thread()
            .post()
            .id(body_id)
            .type_(ForumThreadType::Manga)
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn create_a_forums_thread_handle_403_error() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let body_id = Uuid::new_v4();
        let response_body = json!({
            "result": "error",
            "errors": [
                {
                    "id": "string",
                    "status": 0,
                    "title": "string",
                    "detail": "string"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path_regex("/forums/thread"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({
                "type": "manga",
                "id": body_id
            })))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .forums()
            .thread()
            .post()
            .id(body_id)
            .type_(ForumThreadType::Manga)
            .build()?
            .send()
            .await
            .expect_err("an error should be received");

        Ok(())
    }

    #[tokio::test]
    async fn create_a_forums_thread_handle_404_error() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let body_id = Uuid::new_v4();
        let response_body = json!({
            "result": "error",
            "errors": [
                {
                    "id": "string",
                    "status": 0,
                    "title": "string",
                    "detail": "string"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path_regex("/forums/thread"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({
                "type": "manga",
                "id": body_id
            })))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .forums()
            .thread()
            .post()
            .id(body_id)
            .type_(ForumThreadType::Manga)
            .build()?
            .send()
            .await
            .expect_err("an error should be received");

        Ok(())
    }
}
