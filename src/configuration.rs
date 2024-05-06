use config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Add configuration values from a file named 'configuration'. It will look
    // for any top-level file with an extension that 'config' knows how to
    // parse: yaml, json, etc.
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;

    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
