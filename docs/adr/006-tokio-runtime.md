# ADR-006: Tokio Async Runtime

## Status

Accepted

Date: 2026-01-26

## Context

Rust requires choosing an async runtime for async/await. Requirements:

- **Mature**: Production-ready, well-tested
- **Performance**: Efficient task scheduling
- **Ecosystem**: Library support
- **Features**: Timers, IO, channels, etc.
- **Documentation**: Clear examples
- **Compatibility**: Works with our dependencies

Available async runtimes:
- **Tokio**: Most popular, feature-rich
- **async-std**: Alternative, stdlib-like API
- **smol**: Minimal, simple
- **Custom**: Roll our own (massive effort)

Dependencies requiring async:
- Serenity (Discord) - uses Tokio
- Reqwest (HTTP) - supports Tokio or async-std
- Axum (web server) - built on Tokio

## Decision

We will use **Tokio v1** as our async runtime with full features.

Configuration:
```toml
tokio = { version = "1", features = ["full"] }
```

Usage:
- `#[tokio::main]` for main function
- `#[tokio::test]` for async tests
- `tokio::spawn` for concurrent tasks
- `tokio::select!` for racing operations
- `tokio::sync` for async primitives

## Consequences

### Positive

- **Ecosystem**: Best library support
- **Unified runtime**: One runtime for all components
- **Features**: Everything we need built-in
- **Performance**: Work-stealing scheduler
- **Documentation**: Extensive guides and examples
- **Community**: Large, active community
- **Testing**: Great async test support

### Negative

- **Binary size**: Larger than minimal runtimes
- **Complexity**: Many features we don't use (yet)
- **Lock-in**: Hard to switch runtimes later

### Neutral

- **Feature set**: "full" gives us everything, but could use selective features
- **Learning curve**: async Rust + Tokio concepts

## Notes

Serenity's requirement for Tokio made this decision straightforward - using a different runtime would require running two runtimes simultaneously, adding significant complexity.

The "full" features flag was chosen for MVP simplicity. Future optimization could use selective features:
```toml
# Optimized (future):
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
```

Tokio's work-stealing scheduler handles our workload well:
- Discord event handling (many small tasks)
- HTTP requests to Claude (IO-bound)
- WebSocket connections (long-lived)

## References

- [Tokio Documentation](https://tokio.rs/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- Related: ADR-003 (Serenity) - requires Tokio
- Related: ADR-004 (Axum) - built on Tokio
