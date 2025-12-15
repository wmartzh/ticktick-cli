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

## 🔒 Security

- **Secure Token Storage**: All access tokens are stored in the system keychain (never in plain text files)
- **OAuth2 Flow**: Industry-standard OAuth2 authentication via proxy server
- **HTTPS Only**: All API communication encrypted via HTTPS
- **No Token Logging**: Sensitive data never written to logs
- **Memory Safety**: Built with Rust for memory-safe operations

**Best Practices:**
- Tokens are stored per-user in the system keychain
- `.env` files should never be committed to version control
- The CLI never transmits passwords directly

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

We welcome contributions! Whether it's bug fixes, new features, documentation improvements, or suggestions.

### How to Contribute

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes (please run `cargo fmt` and `cargo clippy`)
4. Commit your changes: `git commit -m "feat: add amazing feature"`
5. Push to your fork: `git push origin feature/amazing-feature`
6. Open a Pull Request

### What We Need

**High Priority:**
- Task update/delete functionality
- Task completion/status management
- Unit and integration tests
- Better error messages

**Nice to Have:**
- Recurring tasks support
- Task priorities and subtasks
- Interactive TUI mode
- Shell completions (bash, zsh, fish)
- Reminders and notifications

### Getting Help

- Open an issue for bugs or feature requests
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
