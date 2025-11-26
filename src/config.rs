use std::{env, sync::OnceLock};

use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub api_host: String,
    pub api_client: String,
    pub csrf_token: String,
    pub server_host: [i16; 4],
    pub server_port: i64,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Config {
            api_key: env::var("API_KEY").expect("API_KEY is missing"),
            api_host: env::var("API_HOST").expect("API_HOST is missing"),
            api_client: env::var("API_CLIENT").expect("API_CLIENT is missing"),
            csrf_token: env::var("CSRF_TOKEN").expect("CSRF_TOKEN is missing"),
            server_host: [127, 0, 0, 1],
            server_port: 49153,
        }
    }
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(Config::init)
}
