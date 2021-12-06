use zero2prod;
use std::net::TcpListener;

#[actix_rt::test]
async fn test_health_check() {
    let response = Client::new()
        .get("/health-check")
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

/// Test client takes care of spinning up server at a random port
pub struct Client {
    client: reqwest::Client,
    url: String,
}

impl Client {
    /// Create test client with specified address
    pub fn new_with(address: &str) -> Self {
        let listener = TcpListener::bind(address).expect("failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = zero2prod::run(listener).expect("failed to bind address");
        let _ = tokio::spawn(server);
        let client = reqwest::Client::new();
        let url = format!("http://127.0.0.1:{}", port);
        Self { client, url }
    }

    /// Create test client at 127.0.0.1 and a random port
    pub fn new() -> Self {
        Self::new_with("127.0.0.1:0")
    }

    /// Create request builder for a GET request at the specified route
    pub fn get(&self, route: &str) -> reqwest::RequestBuilder {
        self.client.get(format!("{}{}", self.url, normalize(route)))
    }
}

fn normalize(route: &str) -> String {
    let mut route = String::from(route);
    if !route.starts_with("/") {
        route = format!("/{}", route);
    }
    route
}
