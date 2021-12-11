use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: App,
    pub database: Database,
}

#[derive(serde::Deserialize)]
pub struct App {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

impl App {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(serde::Deserialize)]
pub struct Database {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub ssl: bool,
}

impl Database {
    pub fn to_options_with(&self, name: Option<&str>) -> PgConnectOptions {
        let mode = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        let options = PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(mode)
            .log_statements(log::LevelFilter::Trace)
            .to_owned();

        if let Some(name) = name {
            options.database(name)
        } else {
            options
        }
    }

    pub fn to_options(&self) -> PgConnectOptions {
        self.to_options_with(Some(&self.name))
    }
}

pub fn load() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let path = std::env::current_dir().expect("failed to determine current directory");
    let dir = path.join("settings");

    // Read base config
    settings.merge(config::File::from(dir.join("base")).required(true))?;

    // Determine environment, defaulting to Dev
    let env: Env = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("failed to parse APP_ENV");

    // Read env-specific config
    settings.merge(config::File::from(dir.join(env.as_str())).required(true))?;

    // Read environment variables prefixed with `APP_`
    settings.merge(config::Environment::with_prefix("APP").separator("_"))?;

    settings.try_into()
}

pub enum Env {
    Dev,
    Prod,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Dev => "dev",
            Env::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "Unrecognized environment {}; use dev or prod",
                other
            )),
        }
    }
}
