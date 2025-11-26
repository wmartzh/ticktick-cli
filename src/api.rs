use std::{io::ErrorKind, sync::OnceLock};

use reqwest::Client;
use tokio::sync::oneshot;
use url::{ParseError, Url};
use uuid::Uuid;

use crate::{config, local_server};

static API_CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    API_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Falied to create HTTP Client")
    })
}

pub async fn authenticate() -> Result<(), ParseError> {
    let scope = String::from("tasks:read tasks:write");
    let redirect_uri = String::from("http://127.0.0.1:49153/callback");
    let state = String::from(&config::get().csrf_token);
    let response_type = String::from("code");
    let params = [
        ("client_id", &config::get().api_client),
        ("redirect_uri", &redirect_uri),
        ("scope", &scope),
        ("state", &state),
        ("response_type", &response_type),
    ];

    let (tx, rx) = oneshot::channel();

    //run local server to listen callback

    let server_handle = tokio::spawn(local_server::run_server(tx));

    println!("Requesting Auth");
    let mut url = Url::parse(&config::get().api_host)?;

    // 3. Set the query parameters
    // This function automatically handles URL encoding of parameter names and values.
    url.query_pairs_mut().extend_pairs(params.iter());

    let auth_url = url.to_string();

    println!("Opening browser to: {}", auth_url);
    if open::that(&auth_url).is_err() {
        eprintln!("Could not automatically open browser. Please visit the URL above manually.");
    }

    match rx.await {
        Ok(code) => {
            println!("\n✅ Authorization successful! Received code: {}", code);
            // NOW: Use 'code' to exchange for a token using reqwest::Client::post()
            // e.g., exchange_token(code).await?;
        }
        Err(_) => {
            eprintln!("\n❌ Authorization failed: Server shut down without receiving the code.");
        }
    }

    // Ensure the server task finishes properly
    let _ = server_handle.await;
    Ok(())
}
