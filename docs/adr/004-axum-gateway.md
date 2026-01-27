# ADR-004: Axum for HTTP Gateway

## Status

Accepted

Date: 2026-01-26

## Context

We need an HTTP server framework for the monitoring/control gateway. Requirements:

- **HTTP routes**: Health check, status endpoints
- **WebSocket support**: Real-time communication
- **Async/await**: Integration with Tokio runtime
- **Type safety**: Request/response type checking
- **Performance**: Low overhead
- **Ergonomics**: Easy to use and extend

Available Rust web frameworks:
- **Axum**: Modern, type-safe, built on Tokio/Tower
- **Actix-web**: Fast, mature, different runtime (Actor model)
- **Rocket**: Ergonomic, but requires nightly Rust
- **Warp**: Filter-based, less intuitive
- **Tide**: Async-std based (not Tokio)

## Decision

We will use **Axum v0.8** for the HTTP/WebSocket gateway.

Configuration:
```toml
axum = { version = "0.8", features = ["ws"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["trace"] }
```

Architecture:
- HTTP routes for monitoring (`/health`)
- WebSocket upgrade for control (`/ws`)
- JSON protocol for commands/responses
- Middleware for logging (tower-http)

## Consequences

### Positive

- **Type safety**: Extractors checked at compile time
- **Tokio integration**: Shares runtime with Serenity
- **Composability**: Tower middleware ecosystem
- **WebSocket**: First-class WS support
- **Ergonomics**: Clean routing, less boilerplate
- **Performance**: Zero-cost abstractions
- **Maintained**: Active development by Tokio team

### Negative

- **Newer framework**: Less battle-tested than Actix
- **Learning curve**: Tower concepts (services, layers)
- **Documentation**: Fewer examples than mature frameworks
- **Breaking changes**: Rapid evolution (0.x versions)

### Neutral

- **Ecosystem**: Growing but smaller than Actix
- **Community**: Smaller but responsive

## Notes

Axum's integration with Tokio was crucial - we already use Tokio for Serenity, so sharing the runtime avoids complexity.

The type-safe extractors align well with Rust's type system and our TDD approach:
```rust
async fn handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    // Compiler ensures types match
}
```

Alternative (Actix) was considered but uses different async runtime (would need two runtimes). Rocket requires nightly Rust (unacceptable for stability).

## References

- [Axum GitHub](https://github.com/tokio-rs/axum)
- [Axum Documentation](https://docs.rs/axum/)
- [Tower Middleware](https://docs.rs/tower/)
- Related: ADR-006 (Tokio Runtime) - Axum built on Tokio
- Related: ADR-009 (WebSocket JSON Protocol)
