use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::settings;
use zero2prod::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::load().expect("failed to read config file");
    let pool = PgPool::connect(&settings.database.url())
        .await
        .expect("failed to connect to postgres");
    let address = format!("127.0.0.1:{}", settings.port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, pool)?.await
}
