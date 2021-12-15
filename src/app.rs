use crate::email::EmailClient;
use crate::routes::{health_check, subscribe};
use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::{self, web, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn new(settings: Settings) -> std::io::Result<Self> {
        let pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(settings.database.to_options());
        let sender = settings
            .email
            .sender()
            .expect("invalid sender email address");
        let timeout = settings.email.timeout();
        let client = EmailClient::new(
            settings.email.base_url,
            sender,
            settings.email.auth_token,
            timeout,
        );

        let listener = TcpListener::bind(settings.app.address())?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool, client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> std::io::Result<()> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, pool: PgPool, client: EmailClient) -> std::io::Result<Server> {
    let pool = web::Data::new(pool);
    let client = web::Data::new(client);
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
            .app_data(client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
