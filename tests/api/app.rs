use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{app::App, settings, telemetry};

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = telemetry::load("test".into(), "debug".into(), std::io::stdout);
        telemetry::init(subscriber);
    } else {
        let subscriber = telemetry::load("test".into(), "debug".into(), std::io::sink);
        telemetry::init(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        Lazy::force(&TRACING);

        let settings = {
            let mut settings = settings::load().expect("failed to read config");
            settings.database.name = Uuid::new_v4().to_string();
            settings.app.port = 0;
            settings
        };

        let app = App::new(settings.clone())
            .await
            .expect("failed to build application");
        let address = format!("http://127.0.0.1:{}", app.port());
        let pool = configure_database(&settings.database).await;
        let _ = tokio::spawn(app.run());

        TestApp { address, pool }
    }
}

async fn configure_database(config: &settings::Database) -> PgPool {
    let mut conn = PgConnection::connect_with(&config.to_options_with(None))
        .await
        .expect("failed to connect to postgres");
    conn.execute(format!("CREATE DATABASE \"{}\";", config.name).as_str())
        .await
        .expect("failed to create database");

    let pool = PgPool::connect_with(config.to_options())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to migrate database");

    pool
}
