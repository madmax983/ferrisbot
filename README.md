# Ferrisbot

A Rust-based Discord bot powered by Claude AI, built with strict TDD principles.

**Current Status:** v0.1.0 - MVP Complete ✅

This is a **foundational implementation** providing core infrastructure for:
- Discord message handling
- Claude API integration
- HTTP/WebSocket gateway
- Basic chat functionality

**Ready to extend with:** Agent framework, tool use, streaming, persistence, advanced conversation management, and more.

## Architecture

Ferrisbot is structured into modular components:

- **`types/`** - Core message types with serialization
- **`error/`** - Unified error handling with `thiserror`
- **`llm/`** - Claude API client with request/response types
- **`discord/`** - Discord bot wrapper and message conversion
- **`gateway/`** - Axum HTTP server with WebSocket support
- **`app`** - Application orchestration layer

## Implementation Status

### ✅ MVP Foundation Complete (v0.1.0)

**Core Infrastructure:**
- **Phase 1: Foundation** - Core types, error handling, serialization
- **Phase 2: Claude API Client** - Full API integration with mocked tests
- **Phase 3: Discord Integration** - Bot setup, message conversion, reply helpers
- **Phase 4: Gateway** - HTTP server with health check and WebSocket endpoints
- **Phase 5: Integration** - Full message flow, Discord event handler, end-to-end tests

### 🚧 Next Steps (Post-MVP)

The foundation is solid. Now we can build:

**Agent Framework:**
- Tool use / function calling
- Multi-agent conversations
- Context management
- Plugin system

**Advanced Features:**
- Streaming responses
- Conversation history/memory
- Database persistence
- Admin commands via gateway
- Channel/role permissions
- Rate limiting per user

**Infrastructure:**
- Metrics and monitoring
- Structured logging improvements
- Configuration management
- Deployment automation

## Test Coverage

```
✅ 48 tests passing (44 unit + 4 integration)
- 8 types/error tests
- 15 LLM client tests
- 10 Discord tests (including handler)
- 8 gateway tests (including WebSocket)
- 2 app tests
- 4 integration tests (end-to-end)
```

### Message Flow

```
Discord User → Discord Message Event
    ↓
BotHandler (filters, converts)
    ↓
Claude API (send_message)
    ↓
Response Processing
    ↓
Discord Reply
```

## Documentation

- **[Quick Start Guide](./QUICKSTART.md)** - Setup and usage
- **[Development Guide](./CLAUDE.md)** - For AI assistants and developers
- **[Roadmap](./ROADMAP.md)** - Feature development plan
- **[Architecture Decision Records](./docs/adr/)** - Why we built it this way
- **[Architecture Diagrams](./docs/diagrams/)** - Visual system documentation

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone <repo-url>
cd ferrisbot
```

### Configuration

Set environment variables:

```bash
export DISCORD_TOKEN="your-discord-bot-token"
export ANTHROPIC_API_KEY="your-claude-api-key"
export GATEWAY_PORT="18789"  # Optional, defaults to 18789
```

### Build and Test

```bash
# Run all tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Build release binary
cargo build --release
```

### Run

```bash
# Start the gateway server
cargo run
```

The gateway will start on `http://127.0.0.1:18789` with:
- Health check: `GET /health`
- WebSocket: `GET /ws` (upgrade)

### Verify

```bash
# Check health endpoint
curl http://127.0.0.1:18789/health
# Expected: OK

# WebSocket (requires ws client)
wscat -c ws://127.0.0.1:18789/ws
```

## Development

This project follows **strict TDD (Test-Driven Development)**:

1. **RED** - Write a failing test
2. **GREEN** - Write minimal code to pass
3. **REFACTOR** - Clean up while keeping tests green

All features were developed following this cycle, resulting in high test coverage and reliable code.

## Project Structure

```
ferrisbot/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library exports
│   ├── main.rs          # Binary entry point
│   ├── app.rs           # App orchestration
│   ├── error.rs         # Error types
│   ├── types/           # Core types
│   │   └── mod.rs
│   ├── llm/             # Claude API client
│   │   ├── mod.rs
│   │   ├── claude.rs
│   │   └── types.rs
│   ├── discord/         # Discord integration
│   │   ├── mod.rs
│   │   ├── bot.rs
│   │   └── convert.rs
│   └── gateway/         # HTTP/WebSocket gateway
│       ├── mod.rs
│       ├── server.rs
│       ├── routes.rs
│       └── ws.rs
└── README.md
```

## Dependencies

- **axum** - HTTP server & WebSocket gateway
- **serenity** - Discord bot framework
- **reqwest** - HTTP client for Claude API
- **tokio** - Async runtime
- **serde/serde_json** - Serialization
- **thiserror/anyhow** - Error handling
- **tracing** - Structured logging

## License

MIT
