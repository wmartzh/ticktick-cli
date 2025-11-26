use std::{env, sync::OnceLock};

use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub api_host: String,
    pub api_client: String,
    pub api_auth_uri: String,
    pub api_token_uri: String,
    pub csrf_token: String,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Config {
            api_key: env::var("API_KEY").expect("API_KEY is missing"),
            api_host: env::var("API_HOST").expect("API_HOST is missing"),
            api_client: env::var("API_CLIENT").expect("API_CLIENT is missing"),
            api_auth_uri: env::var("AUTH_URI").expect("AUTH_URI is missing"),
            api_token_uri: env::var("AUTH_TOKEN_URI").expect("AUTH_TOKEN_URI is missing"),
            csrf_token: env::var("CSRF_TOKEN").expect("CSRF_TOKEN is missing"),
        }
    }
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(Config::init)
}
