use std::env::current_dir;
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSetting
}

#[derive(Deserialize)]
pub struct ApplicationSetting {
    pub port: u16,
    pub host: String
}


#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.as_str())
            .port(self.port)

    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
    }

}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = current_dir()
        .expect("Failed to determine directory");
    let configuration_dir = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");
    let env_filename = format!("{}.yaml", environment.as_str());


    let settings = config::Config::builder()
        .add_source(
            config::File::from(configuration_dir.join("base.yaml"))
        )
        .add_source(
            config::File::from(configuration_dir.join(env_filename))
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(
                format!(
                    "{} is not a supported environement. Use either `local`or `production`."
                    , other
                )
            )
        }
    }
}