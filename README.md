# Tick CLI

A command-line interface tool for authenticating with TickTick using OAuth2.

## Overview

Tick CLI provides a secure OAuth2 authentication flow for TickTick's API. It handles the complete authentication process by:
- Opening your browser to TickTick's authorization page
- Running a local callback server to receive the authorization code
- Exchanging the code for an access token
- Securely storing the token in your system's keychain

## Features

- **OAuth2 Authentication**: Complete OAuth2 authorization code flow
- **Secure Token Storage**: Uses system keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **Local Callback Server**: Temporary server on `http://127.0.0.1:49153` to handle OAuth callbacks
- **Automatic Browser Launch**: Opens authorization URL automatically
- **Clean Shutdown**: Server terminates immediately after successful authentication

## Prerequisites

- Rust 1.70+ (2021 edition)
- TickTick API credentials (Client ID and Client Secret)
- System keychain access

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd tick-cli

# Build the project
cargo build --release

# The binary will be in target/release/tick-cli
```

## Configuration

Create a `.env` file in the project root with your TickTick API credentials:

```env
API_KEY=your_api_secret_here
API_HOST=https://api.ticktick.com
API_CLIENT=your_client_id_here
AUTH_URI=https://ticktick.com/oauth/authorize
AUTH_TOKEN_URI=https://ticktick.com/oauth/token
CSRF_TOKEN=random_string_for_csrf_protection
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `API_KEY` | Your TickTick API secret/client secret | `abc123...` |
| `API_HOST` | TickTick API base URL | `https://api.ticktick.com` |
| `API_CLIENT` | Your TickTick client ID | `xyz789...` |
| `AUTH_URI` | OAuth authorization endpoint | `https://ticktick.com/oauth/authorize` |
| `AUTH_TOKEN_URI` | OAuth token exchange endpoint | `https://ticktick.com/oauth/token` |
| `CSRF_TOKEN` | Random string for CSRF protection | `any_random_string` |

## Usage

### Authenticate with TickTick

```bash
tick-cli --auth
```

or

```bash
tick-cli -a
```

This command will:
1. Start a local OAuth callback server on port 49153
2. Open your browser to TickTick's authorization page
3. Wait for you to authorize the application
4. Receive the authorization code via callback
5. Exchange the code for an access token
6. Save the token securely to your system keychain
7. Shutdown the server and exit

### Token Storage

Access tokens are stored securely using the system's credential manager:
- **macOS**: Keychain
- **Windows**: Credential Manager
- **Linux**: Secret Service (GNOME Keyring, KWallet, etc.)

Service name: `ticktick-cli`
Username: `user-test`

## Architecture

### Project Structure

```
tick-cli/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── api.rs            # OAuth flow and token exchange
│   ├── config.rs         # Configuration management
│   ├── keychain.rs       # Secure credential storage
│   └── local_server.rs   # OAuth callback server
├── Cargo.toml
├── .env                  # Configuration (not committed)
└── README.md
```

### OAuth Flow

1. **Authorization Request**: Opens browser to TickTick OAuth page with parameters
2. **User Authorization**: User logs in and grants permissions
3. **Callback**: TickTick redirects to `http://127.0.0.1:49153/callback?code=...`
4. **Token Exchange**: CLI exchanges authorization code for access token
5. **Storage**: Access token saved to system keychain
6. **Cleanup**: Local server shuts down

### Technical Details

- **HTTP Client**: Built with `reqwest` for async HTTP requests
- **Web Server**: Uses `warp` for the OAuth callback endpoint
- **Async Runtime**: Powered by `tokio`
- **CLI Parsing**: `clap` with derive macros
- **Credential Storage**: `keyring` crate for cross-platform keychain access

## Development

### Build

```bash
cargo build
```

### Run in Development

```bash
cargo run -- --auth
```

### Run Tests

```bash
cargo test
```

## Dependencies

- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `warp` - Web server framework
- `serde` / `serde_json` - Serialization
- `dotenv` - Environment variable management
- `url` - URL parsing and manipulation
- `open` - Cross-platform browser launcher
- `keyring` - System keychain integration
- `uuid` - UUID generation

## Security Considerations

- **CSRF Protection**: Uses state parameter to prevent CSRF attacks
- **Secure Storage**: Tokens stored in system keychain, not in files
- **HTTPS**: All API communication uses HTTPS
- **Local Only**: Callback server only binds to localhost
- **Automatic Cleanup**: Server shuts down immediately after receiving token

## Troubleshooting

### Browser doesn't open automatically

If the browser doesn't open, copy the URL printed in the terminal and open it manually.

### Port 49153 already in use

Make sure no other application is using port 49153. You can change the port in `src/local_server.rs` and update `REDIRECT_URI` in both `src/api.rs` and your OAuth app settings.

### Token storage fails

Ensure you have proper keychain access on your system:
- **macOS**: Grant Terminal/iTerm2 keychain access
- **Linux**: Install and configure a secret service provider
- **Windows**: Run with appropriate user permissions

### Environment variables missing

Ensure your `.env` file exists and contains all required variables. Check `src/config.rs` for the complete list.

## License

[Add your license here]

## Author

wmartz@gmail.com

## Contributing

[Add contributing guidelines here]
