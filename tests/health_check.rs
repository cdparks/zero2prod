use std::net::TcpListener;
use zero2prod;

pub mod health_check {
    #[actix_rt::test]
    async fn test_health_check() {
        let response = super::Client::new()
            .get("/health-check")
            .send()
            .await
            .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(response.content_length(), Some(0));
    }
}

pub mod subscriptions {

    #[actix_rt::test]
    async fn should_200_for_valid_form() {
        let response = super::Client::new()
            .post("/subscriptions")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(response.status().as_u16(), 200);
    }

    #[actix_rt::test]
    async fn should_400_on_missing_data() {
        let cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (body, error) in cases {
            let response = super::Client::new()
                .post("/subscriptions")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
                .expect("failed to execute request");

            assert_eq!(
                response.status().as_u16(),
                400,
                "did not fail with 400 Bad Request despite {}",
                error
            );
        }
    }
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
        let server = zero2prod::startup::run(listener).expect("failed to bind address");
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

    /// Create request builder for a POST request at the specified route
    pub fn post(&self, route: &str) -> reqwest::RequestBuilder {
        self.client
            .post(format!("{}{}", self.url, normalize(route)))
    }
}

fn normalize(route: &str) -> String {
    let mut route = String::from(route);
    if !route.starts_with("/") {
        route = format!("/{}", route);
    }
    route
}
