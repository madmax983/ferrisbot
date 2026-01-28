# Jules Control Plane

A Rust-based Discord control plane powered by Jules (an AI software engineer), built with strict TDD principles.

**Current Status:** v0.1.0 - Foundation Complete ✅

This is a **foundational implementation** providing core infrastructure for:
- Discord message handling
- Jules API integration (via Claude-compatible API)
- HTTP/WebSocket gateway
- Control plane functionality

**Ready to extend with:** Agent framework, tool use, streaming, persistence, advanced conversation management, and more.

## Architecture

Jules Control Plane is structured into modular components:

- **`types/`** - Core message types with serialization
- **`error/`** - Unified error handling with `thiserror`
- **`llm/`** - LLM API client with request/response types
- **`discord/`** - Discord bot wrapper and message conversion
- **`gateway/`** - Axum HTTP server with WebSocket support
- **`app`** - Application orchestration layer

## Implementation Status

### ✅ MVP Foundation Complete (v0.1.0)

**Core Infrastructure:**
- **Phase 1: Foundation** - Core types, error handling, serialization
- **Phase 2: API Client** - Full API integration with mocked tests
- **Phase 3: Discord Integration** - Bot setup, message conversion, reply helpers
- **Phase 4: Gateway** - HTTP server with health check and WebSocket endpoints
- **Phase 5: Integration** - Full message flow, Discord event handler, end-to-end tests

### 🚧 Next Steps

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
✅ Tests passing
- types/error tests
- LLM client tests
- Discord tests (including handler)
- gateway tests (including WebSocket)
- app tests
- integration tests (end-to-end)
```

### Message Flow

```
Discord User → Discord Message Event
    ↓
BotHandler (filters, converts)
    ↓
Jules API (send_message with System Prompt)
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

## Core Dependencies

- **axum** - HTTP server & WebSocket gateway
- **serenity** - Discord bot framework
- **reqwest** - HTTP client for Claude API
- **tokio** - Async runtime
- **serde/serde_json** - Serialization
- **thiserror/anyhow** - Error handling
- **tracing** - Structured logging

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone <repo-url>
cd jules-control-plane
```

### Configuration

Set environment variables:

```bash
export DISCORD_TOKEN="your-discord-bot-token"
export ANTHROPIC_API_KEY="your-api-key"
export JULES_API_URL="https://api.anthropic.com/v1/messages" # Optional, defaults to Anthropic
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

## License

MIT
