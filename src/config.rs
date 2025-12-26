use std::{env, error::Error, sync::OnceLock};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub api_host: String,
    pub auth_host: String,
    pub time_zone: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub email: Option<String>,
    pub default_project: String,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Config {
            app_name: String::from("tick-cli"),
            api_host: env::var("API_HOST").expect("API_HOST is missing"),
            auth_host: env::var("AUTH_HOST").expect("AUTH_HOST is missing"),
            time_zone: iana_time_zone::get_timezone().unwrap_or(String::from("")),
        }
    }
}

impl AppConfig {
    /// Load the user config from disk
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let cfg: AppConfig = confy::load(&get().app_name, None)?;
        Ok(cfg)
    }

    /// Update the config using a closure, then save to disk
    ///
    /// # Example
    /// ```
    /// AppConfig::update(|cfg| {
    ///     cfg.email = Some("user@example.com".to_string());
    ///     cfg.default_project = Some("work".to_string());
    /// })?;
    /// ```
    pub fn update<F>(updater: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut cfg: AppConfig = confy::load(&get().app_name, None)?;
        updater(&mut cfg);
        confy::store(&get().app_name, None, cfg)?;
        Ok(())
    }
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(Config::init)
}
