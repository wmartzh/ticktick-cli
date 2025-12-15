# Tick CLI

> A blazingly fast, secure command-line interface for managing your TickTick tasks from the terminal.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Dual%20MIT%2FCommercial-blue.svg)](LICENSE)

## Overview

Tick CLI brings the power of TickTick task management to your terminal. Built with Rust for speed and reliability, it provides a seamless command-line experience for creating, viewing, and organizing tasks without leaving your development environment.

## ✨ Features

### 🔐 Authentication & Security

- **OAuth2 Authentication**: Secure browser-based OAuth2 flow via proxy server
- **Keychain Integration**: Tokens stored in system keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **Automatic Token Management**: No manual token handling required

### 📝 Task Management

- **Quick Task Creation**: Create tasks with a single command
- **Flexible Due Dates**: Natural date parsing with timezone support
  - Simple dates: `2025-12-25`
  - Time-specific: `2025-12-16 2:00pm`
- **Tag Support**: Organize tasks with comma-separated tags
- **Project Assignment**: Assign tasks to specific projects or inbox
- **Task Viewing**: List tasks by project with formatted tables

### ⚙️ Configuration

- **User Profiles**: Configure email and default project preferences
- **Timezone Awareness**: Automatic timezone detection and conversion
- **Persistent Settings**: Configuration stored locally using `confy`

### 🚀 Performance

- **Async Architecture**: Built on Tokio for concurrent operations
- **Fast Compilation**: Rust's zero-cost abstractions
- **Minimal Dependencies**: Lean codebase for quick startup

## 📦 Installation

### Prerequisites

- **Rust 1.70+** (2021 edition) - [Install Rust](https://rustup.rs/)
- **TickTick Account** - [Sign up](https://ticktick.com/)
- **System Keychain Access** (usually enabled by default)

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/tick-cli.git
cd tick-cli

# Build and install
cargo build --release

# Optional: Install globally (requires sudo on Unix systems)
cargo install --path .

# Or copy to your PATH
cp target/release/tick-cli /usr/local/bin/tick
```

### macOS Code Signing (Recommended)

To avoid keychain authorization prompts on every run:

```bash
# After building, sign the binary
codesign -s - target/release/tick-cli

# Or add to your shell profile (~/.zshrc or ~/.bashrc)
alias tick-build="cargo build --release && codesign -s - target/release/tick-cli"
```

## ⚙️ Configuration

### 1. Environment Setup

Create a `.env` file in the project root:

```env
API_HOST=https://api.ticktick.com
AUTH_HOST=https://your-auth-proxy-server.com
```

| Variable    | Description            | Example                    |
| ----------- | ---------------------- | -------------------------- |
| `API_HOST`  | TickTick API base URL  | `https://api.ticktick.com` |
| `AUTH_HOST` | OAuth proxy server URL | `https://your-proxy.com`   |

### 2. Configure Your Profile

```bash
# Set your email (required for authentication)
tick config --email your.email@example.com

# Set default project (optional)
tick config --project "Work"

# View current configuration
tick config --show
```

## 🚀 Usage

### Authentication

First time setup - authenticate with TickTick:

```bash
tick auth --login
```

This will:

1. Open your browser to authorize the application
2. Wait for you to complete authorization
3. Securely store your access token in the system keychain
4. You're ready to use Tick CLI!

### Creating Tasks

```bash
# Simple task
tick task create "Buy groceries"

# Task with due date
tick task create "Submit report" --due "2025-12-25"

# Task with time
tick task create "Team meeting" --due "2025-12-16 2:00pm"

# Task with project
tick task create "Review PR" --project "Work"

# Task with tags
tick task create "Research topic" --tags "learning,important"

# Complete example
tick task create "Deploy to production" \
  --project "DevOps" \
  --due "2025-12-20 3:00pm" \
  --tags "urgent,deployment"
```

### Viewing Tasks

```bash
# View all tasks in inbox
tick task get

# View tasks in a specific project
tick task get --project "Work"

# View all tasks across projects
tick task get --all
```

### Date Format Examples

Tick CLI supports flexible date inputs:

- **Simple dates**: `2025-12-25`
- **With time**: `2025-12-16 2:00pm` or `2025-12-16 14:00pm`
- **Timezone-aware**: Automatically converts to UTC based on your system timezone

## 🏗️ Architecture

### Project Structure

```
tick-cli/
├── src/
│   ├── main.rs                # CLI entry point and command routing
│   ├── auth.rs               # OAuth2 authentication via proxy
│   ├── client.rs             # HTTP client initialization
│   ├── config.rs             # Configuration and environment management
│   ├── keychain.rs           # Secure credential storage
│   ├── tick_tick_api.rs      # API data models and types
│   ├── ui/
│   │   └── tables.rs         # Task display formatting
│   └── services/
│       ├── mod.rs            # Service module exports
│       ├── projects.rs       # Project-related operations
│       └── tasks.rs          # Task creation and retrieval
├── Cargo.toml                # Dependencies and metadata
├── .env                      # Environment configuration (not committed)
├── LICENSE                   # Dual MIT/Commercial license
└── README.md
```

### Authentication Flow

```
┌─────────┐    1. auth --login    ┌──────────────┐
│   CLI   │─────────────────────>│  Auth Proxy  │
└─────────┘                       └──────────────┘
     │                                    │
     │ 2. Open browser                    │ 3. Redirect to TickTick
     │<───────────────────────────────────│
     │                                    │
     │ 4. User authorizes                 │
     │────────────────────────────────────>│
     │                                    │
     │ 5. Press ENTER                     │
     │────────────────────────────────────>│
     │                                    │
     │ 6. Exchange for token              │
     │<───────────────────────────────────│
     │                                    │
     │ 7. Store in keychain               │
     └────────────────────────────────────┘
```

### Data Flow

```
User Input → CLI Parser (clap) → Command Handler
                                       │
                                       ├─> Auth Service → Keychain
                                       │
                                       ├─> Config Service → Local Storage
                                       │
                                       └─> Task Service → API Client
                                                            │
                                                            ├─> TickTick API
                                                            └─> Project Service
```

### Technical Stack

| Component         | Technology          | Purpose                                 |
| ----------------- | ------------------- | --------------------------------------- |
| **Language**      | Rust 2021           | Type safety, performance, memory safety |
| **Async Runtime** | Tokio               | Concurrent I/O operations               |
| **HTTP Client**   | reqwest             | API communication                       |
| **CLI Framework** | clap v4             | Argument parsing with derive macros     |
| **Serialization** | serde + serde_json  | JSON handling                           |
| **Keychain**      | keyring             | Cross-platform credential storage       |
| **Date/Time**     | chrono + chrono-tz  | Timezone-aware date parsing             |
| **Config**        | confy + dotenv      | User settings and environment vars      |
| **UI Tables**     | comfy-table         | Formatted task display                  |
| **TUI**           | ratatui + dialoguer | Interactive terminal UI components      |

## 🛠️ Development

### Prerequisites for Development

- **Rust toolchain** (rustc, cargo) via [rustup](https://rustup.rs/)
- **Git** for version control
- **Code editor** with Rust support (VS Code with rust-analyzer recommended)

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/tick-cli.git
cd tick-cli

# Install dependencies (automatically handled by Cargo)
cargo fetch

# Create .env file
cp .env.example .env
# Edit .env with your configuration

# Build in debug mode
cargo build

# Run with arguments
cargo run -- task create "Test task"
```

### Development Commands

```bash
# Build debug version (faster compile, slower runtime)
cargo build

# Build release version (slower compile, optimized runtime)
cargo build --release

# Run directly
cargo run -- <command>

# Run tests
cargo test

# Run with verbose output
RUST_LOG=debug cargo run -- <command>

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# View documentation locally
cargo doc --open
```

### Project Commands

```bash
# View dependency tree
cargo tree

# Update dependencies
cargo update

# Audit dependencies for security issues
cargo audit

# Check outdated dependencies
cargo outdated
```

## 🔒 Security

### Security Features

- **Secure Token Storage**: All access tokens are stored in the system keychain (never in plain text files)
- **OAuth2 Flow**: Industry-standard OAuth2 authentication
- **HTTPS Only**: All API communication encrypted via HTTPS
- **No Token Logging**: Sensitive data never written to logs
- **Memory Safety**: Rust's ownership model prevents common vulnerabilities

### Security Best Practices

- Tokens are stored per-user in the system keychain
- `.env` files should never be committed to version control
- The CLI never transmits passwords directly
- All authentication goes through the proxy server

## 🐛 Troubleshooting

### Keychain Authorization Prompts (macOS)

**Problem**: macOS asks for keychain authorization on every run.

**Solution**: Sign the binary after building:

```bash
codesign -s - target/release/tick-cli
```

Or manually allow access in Keychain Access.app:

1. Open **Keychain Access**
2. Find `tick-cli` entry
3. Right-click → **Get Info** → **Access Control**
4. Select "Allow all applications to access this item"

### Authentication Fails

**Problem**: Browser opens but authentication doesn't complete.

**Solutions**:

- Ensure your `.env` file has correct `AUTH_HOST` URL
- Check that you pressed ENTER in the terminal after authorizing in browser
- Verify your email is configured: `tick config --email your@email.com`
- Check network connectivity to the auth proxy server

### Token Storage Fails

**Problem**: Error saving or retrieving tokens from keychain.

**Platform-specific solutions**:

- **macOS**: Grant Terminal/iTerm2 keychain access in System Preferences
- **Linux**: Install `gnome-keyring` or `kwallet` and ensure it's running
- **Windows**: Run terminal with appropriate user permissions

### Date Parsing Issues

**Problem**: Due dates not being set correctly or showing wrong time.

**Solutions**:

- Use format: `YYYY-MM-DD` or `YYYY-MM-DD HH:MMam/pm`
- Examples: `2025-12-25` or `2025-12-16 2:00pm`
- Check your system timezone is set correctly
- The CLI automatically converts local time to UTC

### Project Not Found

**Problem**: Tasks go to inbox even when specifying a project.

**Solutions**:

- Project names are case-insensitive and use partial matching
- Use `tick task get --project "Work"` to verify project exists
- Check your TickTick account has the project created
- Try using just part of the project name

## 🤝 Contributing

We welcome contributions! Whether it's bug fixes, new features, documentation improvements, or suggestions, all contributions are valued.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Run tests and linting**: `cargo test && cargo clippy`
5. **Format code**: `cargo fmt`
6. **Commit with clear messages**: `git commit -m "feat: add amazing feature"`
7. **Push to your fork**: `git push origin feature/amazing-feature`
8. **Open a Pull Request**

### Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, no logic change)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Code Standards

- **Format**: All code must be formatted with `cargo fmt`
- **Linting**: Pass `cargo clippy` with no warnings
- **Tests**: Add tests for new functionality
- **Documentation**: Add inline documentation for public APIs
- **Error Handling**: Use `Result<T, E>` and proper error messages
- **Safety**: Avoid `unwrap()` in production code paths

### Architecture Guidelines

#### Adding New Commands

1. Define command struct in `src/main.rs`:

```rust
#[derive(Args, Debug)]
struct YourCommandArgs {
    #[arg(short, long)]
    your_option: String,
}
```

2. Add to `Commands` enum:

```rust
enum Commands {
    YourCommand(YourCommandArgs),
    // ...
}
```

3. Handle in `main()`:

```rust
match cli.command {
    Commands::YourCommand(args) => {
        // Implementation
    }
}
```

#### Adding New API Endpoints

1. Define data models in `src/tick_tick_api.rs`:

```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YourType {
    pub field: String,
}
```

2. Create service function in `src/services/`:

```rust
pub async fn your_function() -> Result<YourType, Box<dyn std::error::Error>> {
    let response = client::client()
        .get(format!("{}/endpoint", config::get().api_host))
        .send()
        .await?;

    let data: YourType = response.json().await?;
    Ok(data)
}
```

#### Adding Configuration Options

1. Update `AppConfig` in `src/config.rs`:

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub new_field: Option<String>,
}
```

2. Add CLI option in `src/main.rs`
3. Use via `config::AppConfig::load()?`

### Technical Areas for Contribution

#### High Priority

- [ ] Unit and integration tests
- [ ] Error handling improvements
- [ ] Task update/delete functionality
- [ ] Task completion/status management
- [ ] Better error messages

#### Medium Priority

- [ ] Recurring tasks support
- [ ] Task priorities and subtasks
- [ ] Reminders and notifications
- [ ] Batch operations
- [ ] Export/import tasks
- [ ] Search and filtering

#### Nice to Have

- [ ] Interactive TUI mode
- [ ] Task templates
- [ ] Statistics and reports
- [ ] Pomodoro timer integration
- [ ] Plugin system
- [ ] Shell completions (bash, zsh, fish)

### Development Resources

- **TickTick API**: [Official Documentation](https://developer.ticktick.com/)
- **Rust Book**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
- **Tokio Guide**: [tokio.rs/tokio/tutorial](https://tokio.rs/tokio/tutorial)
- **Clap Documentation**: [docs.rs/clap](https://docs.rs/clap)

### Getting Help

- Open an issue for bugs or feature requests
- Tag issues with appropriate labels (`bug`, `enhancement`, `question`)
- Join discussions in GitHub Discussions
- Email: wmartz@gmail.com
- LinkedIn: [wmartzh](https://www.linkedin.com/in/wmartzh)

## 📝 License

This project is dual-licensed:

- **MIT License** for non-commercial use (personal projects, education, open source)
- **Commercial License** required for commercial use

See [LICENSE](LICENSE) file for details.

For commercial licensing inquiries, contact:

- Email: wmartz@gmail.com
- LinkedIn: [wmartzh](https://www.linkedin.com/in/wmartzh)

## 👤 Author

**Wilian Martinez**

- Email: wmartz@gmail.com
- LinkedIn: [wmartzh](https://www.linkedin.com/in/wmartzh)
- GitHub: [@wmartzh](https://github.com/wmartzh)

## 🙏 Acknowledgments

- TickTick for providing the API
- The Rust community for excellent tooling and support
- All contributors who help improve this project

---

**Built with ❤️ and Rust**
