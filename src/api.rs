use std::{collections::HashMap, sync::OnceLock};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use url::{ParseError, Url};

use crate::{config, keychain::CredentialStore, local_server};

static API_CLIENT: OnceLock<Client> = OnceLock::new();
const REDIRECT_URI: &str = "http://127.0.0.1:49153/callback";
const SCOPE: &str = "tasks:read tasks:write";
const GRANT_TYPE: &str = "authorization_code";

#[derive(Debug, Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
}

fn get_client() -> &'static Client {
    API_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Falied to create HTTP Client")
    })
}

async fn exchange_token(code: String) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let mut form = HashMap::new();

    form.insert("redirect_uri", REDIRECT_URI);
    form.insert("scope", SCOPE);
    form.insert("code", &code);
    form.insert("grant_type", GRANT_TYPE);

    let res = get_client()
        .post(&config::get().api_token_uri)
        .basic_auth(&config::get().api_client, Some(&config::get().api_key))
        .form(&form)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let error_text = res.text().await?;
        return Err(format!("Token Exchange falied: {}", error_text).into());
    }

    let token_response: TokenResponse = res.json().await?;
    println!("{:#?}", token_response);
    Ok(token_response)
}

pub async fn authenticate() -> Result<(), ParseError> {
    let state = String::from(&config::get().csrf_token);
    let response_type = String::from("code");

    let params = [
        ("client_id", config::get().api_client.as_str()),
        ("redirect_uri", REDIRECT_URI),
        ("scope", SCOPE),
        ("state", state.as_str()),
        ("response_type", response_type.as_str()),
    ];

    let (tx, rx) = oneshot::channel();

    //run local server to listen callback

    let server_handle = tokio::spawn(local_server::run_server(tx));

    println!("Requesting Auth");
    let mut url = Url::parse(&config::get().api_auth_uri)?;

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
            eprintln!("Authenticating...");
            let token_response = exchange_token(code).await.unwrap();
            let _ = CredentialStore::save(&token_response.access_token);

            // Shutdown the server now that we have the code
            server_handle.abort();
        }

        Err(_) => {
            eprintln!("\n❌ Authorization failed: Server shut down without receiving the code.");
            server_handle.abort();
        }
    }

    Ok(())
}
