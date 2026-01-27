# ADR-003: Serenity for Discord Integration

## Status

Accepted

Date: 2026-01-26

## Context

We need a Discord library for Rust to handle bot connections, message events, and API interactions. Requirements:

- **Gateway support**: WebSocket connection to Discord
- **Event handling**: React to messages, reactions, etc.
- **Type safety**: Strong typing for Discord types
- **Active maintenance**: Regular updates for Discord API changes
- **Documentation**: Clear examples and API docs
- **Community**: Support and examples available

Available Rust Discord libraries:
- **Serenity**: Most mature, strongly typed, active development
- **Twilight**: Modular design, lower-level, more complex
- **Discord.rs**: Older, less maintained
- **Custom implementation**: Full control, massive effort

## Decision

We will use **Serenity v0.12** for all Discord integration.

Configuration:
```toml
serenity = { version = "0.12", features = ["client", "gateway", "model"] }
```

Key features used:
- `Client` - Discord bot client
- `EventHandler` - Message event handling
- `GatewayIntents` - Permission configuration
- Strongly-typed Discord types (Message, User, Channel, etc.)

## Consequences

### Positive

- **Mature library**: Well-tested, production-ready
- **Strong typing**: Compiler catches Discord API misuse
- **Active community**: Questions answered, issues addressed
- **Good docs**: Examples for common use cases
- **Event system**: Clean handler pattern for messages
- **Type conversions**: Rich type system for Discord entities

### Negative

- **Heavy dependency**: Large dependency tree
- **Learning curve**: Need to understand Serenity's event model
- **Version lock**: Breaking changes between major versions
- **Opinionated**: Less control than lower-level libraries

### Neutral

- **Performance**: Good enough for our scale, not optimized for massive bots
- **Features**: More than we need for MVP, but room to grow

## Notes

Serenity's strongly-typed event handlers integrate well with Rust's type system and our TDD approach. The `EventHandler` trait provides a clean abstraction for message handling.

The library's maturity (started 2016, active development) gives confidence for long-term maintenance.

Alternative (Twilight) was considered but deemed too complex for MVP. May revisit for v2.0+ if we need more control.

## References

- [Serenity GitHub](https://github.com/serenity-rs/serenity)
- [Serenity Documentation](https://docs.rs/serenity/)
- [Discord API Documentation](https://discord.com/developers/docs)
- Related: ADR-006 (Tokio Runtime) - Serenity uses Tokio
