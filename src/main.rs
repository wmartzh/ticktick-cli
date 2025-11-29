use clap::{Args, Parser, Subcommand};

use crate::{keychain::CredentialStore, tick_tick_api::TickTickApi};

mod auth;
mod client;
mod config;
mod keychain;
mod services;
mod tick_tick_api;

#[derive(Args, Debug)]
struct CreateArgs {
    /// Task description (positional argument)
    description: String,

    #[arg(short, long)]
    project: Option<String>,
}

#[derive(Subcommand)]
enum TaskCommands {
    Create(CreateArgs),
}

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

        #[arg(short, long)]
        show: bool,
    },
    Task {
        #[command(subcommand)]
        action: TaskCommands,
    },
}

#[derive(Parser)]
#[command(name = "tick")]
#[command(about = "Tick Tick CLI App", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Initialize the API client with credentials from keychain
/// Only call this when you need to make authenticated API calls
fn init_authenticated_client() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::AppConfig::load()?;

    let email = cfg
        .email
        .ok_or("No email configured. Run: tick config --email <your-email>")?;
    let token = CredentialStore::get(&email)?;

    client::init_client(&token)?;
    Ok(())
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
        Commands::Config {
            email,
            project,
            show,
        } => {
            if let Some(email) = email {
                config::AppConfig::update(|cfg| {
                    cfg.email = Some(email.clone());
                })?;
                println!("✅ Email saved: {}", email);
            } else if let Some(project) = project {
                config::AppConfig::update(|cfg| {
                    cfg.default_project = Some(project.clone());
                })?;
                println!("✅ Default project saved: {}", project);
            } else if show {
                println!("{:#?}", config::AppConfig::load()?);
            }
        }
        Commands::Task { action } => {
            // Only initialize client when making API calls
            init_authenticated_client()?;

            match action {
                TaskCommands::Create(args) => {
                    let project_id: Option<String> =
                        services::projects::get_project_id(args.project)
                            .await
                            .unwrap_or(None);

                    TickTickApi::create(args.description, project_id).await?;
                    println!("✅ Task added successfully")
                }
            }
        }
    }

    Ok(())
}
