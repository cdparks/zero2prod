use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::{settings, startup};

pub mod health_check {
    #[actix_rt::test]
    async fn test_health_check() {
        let address = super::spawn();

        let response = reqwest::Client::new()
            .get(format!("{}/health-check", address))
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
        let address = super::spawn();
        let mut conn = super::connect().await;

        let response = reqwest::Client::new()
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
            .send()
            .await
            .expect("failed to execute request");
        assert_eq!(response.status().as_u16(), 200);

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&mut conn)
            .await
            .expect("failed to fetch saved subscription");
        assert_eq!(saved.email, "ursula_le_guin@gmail.com");
        assert_eq!(saved.name, "le guin");
    }

    #[actix_rt::test]
    async fn should_400_on_missing_data() {
        let address = super::spawn();
        let client = reqwest::Client::new();

        let cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (body, error) in cases {
            let response = client
                .post(format!("{}/subscriptions", address))
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

fn spawn() -> String {
    let address = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", address)).expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", address, port)
}

async fn connect() -> PgConnection {
    let settings = settings::load().expect("failed to read config");
    PgConnection::connect(&settings.database.as_url())
        .await
        .expect("failed to connect to postgres")
}
