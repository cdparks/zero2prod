#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub database: Database,
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub port: u16,
    pub host: String,
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Database {
    pub fn as_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

pub fn load() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config"))?;
    settings.try_into()
}
