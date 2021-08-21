use figment::providers::{Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SmtpConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}


#[derive(Debug, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub smtp: SmtpConfig,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("error loading config")
    }
}
