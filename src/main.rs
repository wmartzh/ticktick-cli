use clap::{Parser, Subcommand};

use crate::keychain::CredentialStore;
mod auth;
mod config;
mod keychain;

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[arg(short, long)]
        login: bool,
        #[arg(short, long)]
        email: Option<String>,
    },
}

#[derive(Parser)]
#[command(name = "tick")]
#[command(about = "Tick Tick CLI App", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Main entry point
/// The #[tokio::main] macro sets up async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { login, email } => {
            if let Some(email) = email {
                config::AppConfig::set_email(email)?;
            }
            if login {
                auth::authenticate(config::AppConfig::get_config().email).await?;
            }
        }
    }

    Ok(())
}
