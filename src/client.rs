use std::{sync::OnceLock, time::Duration};

use reqwest::{
    header::{self},
    Client,
};

static API_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn init_client(token: &str) -> Result<(), reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid token"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    API_CLIENT.set(client).ok();
    Ok(())
}

pub fn client() -> &'static Client {
    API_CLIENT.get().expect("Client not initialized")
}
