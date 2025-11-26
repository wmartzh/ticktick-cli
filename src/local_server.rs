use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{oneshot, Mutex};
use warp::Filter;

/// OAuth callback handler
/// Receives the authorization code from the OAuth provider
async fn callback_handler(
    query: HashMap<String, String>,
    tx: Arc<Mutex<Option<oneshot::Sender<String>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("📨 OAuth callback received");
    println!("Query parameters: {:#?}", query);

    if let Some(code) = query.get("code") {
        println!("✅ Authorization code received: {}", code);

        // Send the code through the oneshot channel
        let mut tx_guard = tx.lock().await;
        if let Some(sender) = tx_guard.take() {
            let _ = sender.send(code.clone());
        }

        let response = json!({
            "message": "Successfully authorized! You can close this window and return to the terminal.",
            "status": "success"
        });
        Ok(warp::reply::json(&response))
    } else {
        println!("❌ Authorization failed - no code in callback");
        let response = json!({
            "message": "Authorization failed or code not found.",
            "status": "error"
        });
        Ok(warp::reply::json(&response))
    }
}

/// Run the local OAuth callback server
/// Returns the authorization code via oneshot channel
pub async fn run_server(tx: oneshot::Sender<String>) {
    println!("🚀 Starting OAuth callback server on http://127.0.0.1:49153");

    // Wrap the sender in Arc<Mutex<Option<>>> so it can be safely shared
    let tx = Arc::new(Mutex::new(Some(tx)));

    // Clone for use in the route
    let tx_filter = warp::any().map(move || tx.clone());

    // Define the callback route
    let callback_route = warp::path!("callback")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(tx_filter)
        .and_then(callback_handler);

    let addr: SocketAddr = ([127, 0, 0, 1], 49153).into();

    println!("📡 Server listening for OAuth callback at http://127.0.0.1:49153/callback");

    // Run the server
    warp::serve(callback_route).run(addr).await;
}
