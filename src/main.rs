use clap::{Parser, Subcommand};

mod auth;
mod config;
mod keychain;

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[arg(short, long)]
        login: bool,
    },
    Config {
        #[arg(short, long)]
        email: Option<String>,

        #[arg(short, long)]
        project: Option<String>,
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
        Commands::Auth { login } => {
            if login {
                auth::authenticate(config::AppConfig::load()?.email).await?;
            }
        }
        Commands::Config { email, project } => {
            if let Some(email) = email {
                config::AppConfig::update(|cfg| {
                    cfg.email = Some(email.clone());
                })?;
            } else if let Some(project) = project {
                config::AppConfig::update(|cfg| {
                    cfg.default_project = Some(project.clone());
                })?;
            }
        }
    }

    Ok(())
}
