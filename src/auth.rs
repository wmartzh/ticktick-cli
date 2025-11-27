use std::sync::OnceLock;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::{config, keychain::CredentialStore};

static API_CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct AuthResponse {
    redirect: String,
    req: String,
    email: String,
}

fn get_client() -> &'static Client {
    API_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Falied to create HTTP Client")
    })
}

async fn exchange_token(
    req: String,
    email: String,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/auth/token?req={}&email={}",
        &config::get().auth_host,
        req,
        email
    );

    let res = get_client().get(url).send().await?;

    let status = res.status();

    if !status.is_success() {
        let error_text = res.text().await?;
        return Err(format!("Token Exchange falied: {}", error_text).into());
    }

    let token_response: TokenResponse = res.json().await?;
    println!("{:#?}", token_response);
    Ok(token_response)
}

pub async fn authenticate(email: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Authenticating 🔐...");

    if email.is_none() {
        eprint!("Please set your email with <command> login --email=<your email>");
        return Ok(());
    }

    let user_email = email.unwrap();

    let url = format!(
        "{}/auth/authorize?email={}",
        &config::get().auth_host,
        user_email
    );

    //run local server to listen callback

    let auth_res = get_client().get(url).send().await?;

    if !auth_res.status().is_success() {
        let error_text = auth_res.text().await?;
        return Err(format!("Auth falied: {}", error_text).into());
    }

    let data_response: AuthResponse = auth_res.json().await?;

    // This function automatically handles URL encoding of parameter names and values.

    let browser = open::that(data_response.redirect.clone());
    if browser.is_err() {
        println!("Opening browser to: {}", data_response.redirect.clone());
        eprintln!("Could not automatically open browser. Please visit the URL above manually.");
    }
    println!("\nPress ENTER when you're done in the browser...");

    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("Verifying token...");
    let token_res: TokenResponse = exchange_token(data_response.req, data_response.email).await?;

    let _ = CredentialStore::save(&user_email, &token_res.access_token)?;

    println!("✅ Auth completed");

    Ok(())
}
