# Ferrisbot Quick Start Guide

## Setup

### 1. Get Discord Bot Token

1. Go to https://discord.com/developers/applications
2. Create a new application
3. Go to "Bot" section and create a bot
4. Copy the bot token
5. Enable "Message Content Intent" under Privileged Gateway Intents
6. Go to OAuth2 → URL Generator
7. Select scopes: `bot`
8. Select permissions: `Send Messages`, `Read Messages/View Channels`, `Read Message History`
9. Use the generated URL to invite the bot to your server

### 2. Get Claude API Key

1. Go to https://console.anthropic.com/
2. Sign up/login to your account
3. Go to API Keys section
4. Create a new API key
5. Copy the key (starts with `sk-ant-`)

### 3. Configure Environment

Create a `.env` file in the project root:

```bash
DISCORD_TOKEN=your_discord_bot_token_here
ANTHROPIC_API_KEY=your_claude_api_key_here
GATEWAY_PORT=18789  # Optional, defaults to 18789
```

Or export them directly:

```bash
export DISCORD_TOKEN="your_discord_bot_token_here"
export ANTHROPIC_API_KEY="your_claude_api_key_here"
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run tests to verify everything works
cargo test

# Start the bot
cargo run --release
```

You should see output like:

```
2025-01-26T... INFO ferrisbot: Starting Ferrisbot...
2025-01-26T... INFO ferrisbot::app: Starting Discord bot...
2025-01-26T... INFO ferrisbot::app: Starting gateway on 127.0.0.1:18789
2025-01-26T... INFO ferrisbot::discord::handler: Ferrisbot is connected and ready!
```

## Usage

### Discord

Send a message in any channel where the bot has access:

```
User: Hello!
Bot: Hello! How can I help you today?

User: What is the capital of France?
Bot: The capital of France is Paris.
```

### Gateway

The gateway provides two endpoints:

#### Health Check

```bash
curl http://127.0.0.1:18789/health
# Response: OK
```

#### WebSocket

Connect via WebSocket for real-time communication:

```bash
# Using wscat (install: npm install -g wscat)
wscat -c ws://127.0.0.1:18789/ws

# Send commands:
> {"type":"ping"}
< {"type":"pong"}

> {"type":"status"}
< {"type":"status","bot_connected":true}
```

## Troubleshooting

### Bot doesn't respond

1. Check Message Content Intent is enabled in Discord Developer Portal
2. Verify bot has proper permissions in the server
3. Check logs for errors
4. Ensure `DISCORD_TOKEN` is correct

### API errors

1. Verify `ANTHROPIC_API_KEY` is correct
2. Check you have API credits available
3. Check network connectivity
4. Review Claude API status: https://status.anthropic.com/

### Gateway won't start

1. Check port 18789 isn't already in use: `netstat -an | grep 18789`
2. Try a different port via `GATEWAY_PORT` environment variable
3. Check firewall settings

## Development

### Run Tests

```bash
# All tests
cargo test

# Specific module
cargo test discord

# Integration tests only
cargo test --test full_flow

# With output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

### Logging

Set log level via `RUST_LOG`:

```bash
# Debug level
RUST_LOG=debug cargo run

# Info level (default)
RUST_LOG=info cargo run

# Specific module
RUST_LOG=ferrisbot::discord=debug cargo run
```

## Architecture

```
┌─────────────────┐
│  Discord User   │
└────────┬────────┘
         │ Message
         ▼
┌─────────────────┐
│  Discord Bot    │ (Serenity)
│   + Handler     │
└────────┬────────┘
         │ Convert & Filter
         ▼
┌─────────────────┐
│  Claude Client  │ (Reqwest)
└────────┬────────┘
         │ HTTP API Call
         ▼
┌─────────────────┐
│  Claude API     │
└────────┬────────┘
         │ Response
         ▼
┌─────────────────┐
│  Discord Reply  │
└─────────────────┘

Parallel Process:
┌─────────────────┐
│  Gateway        │ (Axum)
│  /health        │
│  /ws            │
└─────────────────┘
```

## Next Steps

- Add conversation history/context
- Implement channel filtering
- Add rate limiting
- Add admin commands via WebSocket
- Store chat history in database
- Add more Claude features (streaming, multi-turn)

## Support

For issues or questions:
- Check existing tests in `tests/` for examples
- Review inline documentation in source code
- Check logs with `RUST_LOG=debug`
