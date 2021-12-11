use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::settings;
use zero2prod::startup;
use zero2prod::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::load("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init(subscriber);

    let settings = settings::load().expect("failed to read config file");
    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(settings.database.to_options());

    let listener = TcpListener::bind(settings.app.address())?;

    startup::run(listener, pool)?.await
}
