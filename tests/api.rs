use serde_json::json;
use std::{env, error::Error, net::IpAddr};
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};
use zerobounce::{Api, ResponseType};

const TEST_KEY: &str = "test-key";

fn local_api(server: &MockServer) -> Api {
    let mut api = Api::new(TEST_KEY);
    api.set_api_url(server.uri());
    api
}

fn live_api() -> Option<Api> {
    env::var("ZEROBOUNCE_API_KEY")
        .ok()
        .filter(|key| !key.trim().is_empty())
        .map(Api::new)
}

fn success<T>(result: Result<ResponseType<T>, Box<dyn Error>>) -> T {
    match result {
        Ok(ResponseType::Success(response)) => *response,
        Ok(ResponseType::Error(error)) => panic!("ZeroBounce returned an error: {}", error.error),
        // A request URL can include a live API key.
        Err(error) => match error.downcast::<reqwest::Error>() {
            Ok(error) => panic!("Request failed: {}", error.without_url()),
            Err(error) => panic!("Request failed: {}", error),
        },
    }
}

#[tokio::test]
async fn get_credits() {
    let server = MockServer::start().await;
    let mut api = local_api(&server);
    for (key, balance) in [("empty-account", 0), ("funded-account", 42)] {
        Mock::given(method("GET"))
            .and(path("/getcredits"))
            .and(query_param("api_key", key))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "Credits": balance.to_string() })),
            )
            .expect(1)
            .mount(&server)
            .await;

        api.set_api_key(key);
        assert_eq!(success(api.get_credits().await).get_credits(), balance);
    }

    if let Some(api) = live_api() {
        success(api.get_credits().await);
    }
}

async fn mock_validation(server: &MockServer, email: &str, ip: Option<IpAddr>, status: &str) {
    let request = Mock::given(method("GET"))
        .and(path("/validate"))
        .and(query_param("api_key", TEST_KEY))
        .and(query_param("email", email));
    let request = match ip {
        Some(ip) => request.and(query_param("ip_address", ip.to_string())),
        None => request.and(query_param_is_missing("ip_address")),
    };
    request
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "address": email,
            "status": status,
            "sub_status": "",
            "free_email": false,
            "processed_at": "2022-01-01 00:00:00"
        })))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn validate() {
    let server = MockServer::start().await;
    let local = local_api(&server);
    let live = live_api();

    for (label, wire_status) in [
        ("valid", "valid"),
        ("invalid", "invalid"),
        ("catch_all", "catch-all"),
        ("unknown", "unknown"),
        ("spamtrap", "spamtrap"),
        ("abuse", "abuse"),
    ] {
        let email = format!("{}@example.com", label);
        mock_validation(&server, &email, None, wire_status).await;
        for api in std::iter::once(&local).chain(live.as_ref()) {
            let response = success(api.validate(email.as_str(), None).await);
            assert_eq!(response.status.as_str(), label, "{}", email);
        }
    }
}

#[tokio::test]
async fn validate_with_ip() {
    let server = MockServer::start().await;
    let api = local_api(&server);
    let email = "valid@example.com";
    let ip = Some(IpAddr::from([127, 0, 0, 1]));
    mock_validation(&server, email, ip, "valid").await;

    let response = success(api.validate(email, ip).await);
    assert_eq!(response.address, email);
    assert!(response.status.is_valid());
}
