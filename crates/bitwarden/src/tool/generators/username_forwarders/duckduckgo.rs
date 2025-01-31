use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::error::Result;
pub async fn generate(http: &reqwest::Client, token: String) -> Result<String> {
    generate_with_api_url(http, token, "https://quack.duckduckgo.com".into()).await
}

async fn generate_with_api_url(
    http: &reqwest::Client,
    token: String,
    api_url: String,
) -> Result<String> {
    let response = http
        .post(format!("{api_url}/api/email/addresses"))
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err("Invalid DuckDuckGo API token".into());
    }

    // Throw any other errors
    response.error_for_status_ref()?;

    #[derive(serde::Deserialize)]
    struct Response {
        address: String,
    }
    let response: Response = response.json().await?;

    Ok(format!("{}@duck.com", response.address))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    #[tokio::test]
    async fn test_mock_server() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        let (server, _client) = crate::util::start_mock(vec![
            // Mock the request to the DDG API, and verify that the correct request is made
            Mock::given(matchers::path("/api/email/addresses"))
                .and(matchers::method("POST"))
                .and(matchers::header("Content-Type", "application/json"))
                .and(matchers::header("Authorization", "Bearer MY_TOKEN"))
                .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                    "address": "bw7prt"
                })))
                .expect(1),
            // Mock an invalid token request
            Mock::given(matchers::path("/api/email/addresses"))
                .and(matchers::method("POST"))
                .and(matchers::header("Content-Type", "application/json"))
                .and(matchers::header("Authorization", "Bearer MY_FAKE_TOKEN"))
                .respond_with(ResponseTemplate::new(401))
                .expect(1),
        ])
        .await;

        let address = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_TOKEN".into(),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap();
        assert_eq!(address, "bw7prt@duck.com");

        let fake_token_error = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_FAKE_TOKEN".into(),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap_err();

        assert!(fake_token_error
            .to_string()
            .contains("Invalid DuckDuckGo API token"));

        server.verify().await;
    }
}
