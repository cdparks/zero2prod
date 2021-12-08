use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{settings, startup, telemetry};

pub struct App {
    pub address: String,
    pub pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = telemetry::load("test".into(), "debug".into(), std::io::stdout);
        telemetry::init(subscriber);
    } else {
        let subscriber = telemetry::load("test".into(), "debug".into(), std::io::sink);
        telemetry::init(subscriber);
    }
});

pub async fn spawn() -> App {
    Lazy::force(&TRACING);

    let localhost = "127.0.0.1";
    let listener =
        TcpListener::bind(format!("{}:0", localhost)).expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", localhost, port);

    let mut settings = settings::load().expect("failed to read config");
    settings.database.name = Uuid::new_v4().to_string();
    let pool = configure_database(&settings.database).await;

    let server = startup::run(listener, pool.clone()).expect("failed to bind address");
    let _ = tokio::spawn(server);

    App { address, pool }
}

pub async fn configure_database(config: &settings::Database) -> PgPool {
    let mut conn = PgConnection::connect(&config.anon_url())
        .await
        .expect("failed to connect to postgres");
    conn.execute(format!("CREATE DATABASE \"{}\";", config.name).as_str())
        .await
        .expect("failed to create database");

    let pool = PgPool::connect(&config.url())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to migrate database");

    pool
}
