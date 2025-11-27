use std::{env, error::Error, sync::OnceLock};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub api_host: String,
    pub auth_host: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub email: Option<String>,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Config {
            app_name: "tick-cli".to_string(),
            api_host: env::var("API_HOST").expect("API_HOST is missing"),
            auth_host: env::var("AUTH_HOST").expect("AUTH_HOST is missing"),
        }
    }
}

impl AppConfig {
    pub fn get_config() -> AppConfig {
        confy::load(&get().app_name, None).unwrap()
    }

    pub fn set_email(email: String) -> Result<(), Box<dyn Error>> {
        let mut cfg: AppConfig = confy::load(&get().app_name, None)?;

        cfg.email = Some(email);

        confy::store(&get().app_name, None, cfg)?;
        Ok(())
    }
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(Config::init)
}
