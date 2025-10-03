use derive_builder::Builder;
use mangadex_api_schema::v5::MangaRecommendationCollection;
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
#[non_exhaustive]
pub struct GetMangaRecommendations {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,
}

endpoint! {
    GET ("/manga/{}/recommendation", manga_id),
    #[no_data] GetMangaRecommendations,
    #[flatten_result] crate::Result<MangaRecommendationCollection>,
    GetMangaRecommendationsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::RelationshipType;

    #[tokio::test]
    async fn get_manga_recommendation_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({"result":"ok","response":"collection","data":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_07b0a382-325b-4fcb-a5d3-6a3047e1064e","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"07b0a382-325b-4fcb-a5d3-6a3047e1064e","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_0ce3ef0f-20fa-4b88-9b7f-9a7a5176193d","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"0ce3ef0f-20fa-4b88-9b7f-9a7a5176193d","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_0e606591-3631-4088-9fe0-7ef726a6a6ed","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"0e606591-3631-4088-9fe0-7ef726a6a6ed","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_1e750911-912e-45de-91ec-f3308b11a927","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"1e750911-912e-45de-91ec-f3308b11a927","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_2650c555-9131-42da-afa6-1025272a70b5","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"2650c555-9131-42da-afa6-1025272a70b5","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_2847472c-e4f0-4280-8b13-06cba7d4bb7f","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"2847472c-e4f0-4280-8b13-06cba7d4bb7f","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_3c3f2d55-1d6c-421b-ad29-abe05c8d67f5","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"3c3f2d55-1d6c-421b-ad29-abe05c8d67f5","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_4e8fa4b5-c472-4191-8435-d1a0706c3e10","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"4e8fa4b5-c472-4191-8435-d1a0706c3e10","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_5c5f2da6-52e7-47b7-b6de-97e21b74a741","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"5c5f2da6-52e7-47b7-b6de-97e21b74a741","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_85332c4d-2ab8-4aff-a2ef-761e687b8f31","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"85332c4d-2ab8-4aff-a2ef-761e687b8f31","type":"manga"}]},{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa_b9367c11-f14d-4118-ba05-1ef74647d80b","type":"manga_recommendation","attributes":{"score":1.0},"relationships":[{"id":"14c3f01e-c8d3-4a34-935b-148cff022daa","type":"manga"},{"id":"b9367c11-f14d-4118-ba05-1ef74647d80b","type":"manga"}]}],"limit":50,"offset":0,"total":11});

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/recommendation"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .id(Uuid::parse_str("14c3f01e-c8d3-4a34-935b-148cff022daa")?)
            .recommendation()
            .get()
            .send()
            .await?;

        assert_eq!(res.total, 11);
        assert_eq!(res.data[0].type_, RelationshipType::MangaRecommendation);

        Ok(())
    }
}
