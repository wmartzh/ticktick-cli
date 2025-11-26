use clap::Parser;

mod api;
mod config;
mod local_server;

/// Simple CLI tool with a local web server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start the local server
    #[arg(short, long)]
    auth: bool,

    #[arg(short, long)]
    test: Option<String>,
}

/// Main entry point
/// The #[tokio::main] macro sets up async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file first

    // Load environment variables

    // Parse command line arguments
    let args = Args::parse();

    if args.auth {
        api::authenticate().await;
    }

    Ok(())
}
