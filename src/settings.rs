#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: App,
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
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    pub fn anon_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(serde::Deserialize)]
pub struct App {
    pub port: u16,
    pub host: String,
}

impl App {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
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
