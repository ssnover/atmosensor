use std::path::PathBuf;

use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub device: DeviceConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub org: String,
    pub bucket: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub token: Secret<String>,
}

impl DatabaseConfig {
    pub fn make_client(&self) -> influxdb2::Client {
        influxdb2::Client::new(
            format!("http://{}:{}", self.host, self.port),
            &self.org,
            self.token.expose_secret(),
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DeviceConfig {
    pub tty_path: PathBuf,
    pub location: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub altitude: u16,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let config_path = std::env::current_dir().expect("Could not get the current working directory");
    let config_path = config_path.join("config");

    let config = config::Config::builder()
        .add_source(config::File::from(config_path.join("atmosensor.yaml")))
        .add_source(
            config::Environment::with_prefix("ATMOS")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    config.try_deserialize::<Config>()
}
