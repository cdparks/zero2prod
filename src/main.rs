use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::settings;
use zero2prod::startup;
use zero2prod::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::load("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init(subscriber);

    let settings = settings::load().expect("failed to read config file");
    let pool = PgPool::connect(&settings.database.url())
        .await
        .expect("failed to connect to postgres");

    let address = format!("127.0.0.1:{}", settings.port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, pool)?.await
}
