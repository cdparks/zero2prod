use zero2prod::settings;
use zero2prod::app::App;
use zero2prod::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::load("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init(subscriber);

    let settings = settings::load().expect("failed to read config file");
    let app = App::new(settings).await?;
    app.run().await?;
    Ok(())
}
