# ADR-005: Single Error Type with thiserror

## Status

Accepted

Date: 2026-01-26

## Context

We need an error handling strategy that:
- Provides context for debugging
- Enables easy error propagation with `?`
- Supports different error sources (IO, HTTP, Discord, Claude)
- Works well with Result<T, E>
- Allows conversion from library errors

Alternatives considered:
- **Multiple error types**: One per module (complex conversions)
- **anyhow::Error**: Great for apps, loses type information
- **Custom error enum**: Full control, lots of boilerplate
- **thiserror**: Derive macros, minimal boilerplate

## Decision

We will use a **single error enum** (`FerrisError`) with **thiserror** for derives.

Implementation:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FerrisError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Discord error: {0}")]
    Discord(String),

    #[error("Claude API error: {0}")]
    Claude(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

pub type Result<T> = std::result::Result<T, FerrisError>;
```

Pattern:
- Use `Result<T>` everywhere fallible
- Automatic `From` conversions for std errors
- Manual variants for domain errors
- `?` operator for propagation

## Consequences

### Positive

- **Simple**: One error type, easy to understand
- **Ergonomic**: `?` operator works seamlessly
- **Automatic conversions**: `#[from]` generates From impls
- **Context**: Error messages include source
- **Type information**: Can match on specific errors
- **Minimal boilerplate**: thiserror handles Display/Error

### Negative

- **Less granular**: Can't distinguish error sources in type system
- **Growing enum**: New error types added over time
- **Lost details**: Some context from original errors

### Neutral

- **Library errors**: Must convert to FerrisError variants
- **Error matching**: Useful for specific handling, not always needed

## Notes

Chose `thiserror` over `anyhow` because:
- Library code (not just application)
- Want to preserve error types for matching
- Consumers can match on error variants

The single error type simplifies function signatures:
```rust
// Instead of:
fn foo() -> Result<T, IoOrJsonOrHttpError> { }

// We have:
fn foo() -> Result<T> { }
```

This aligns with Rust's philosophy: errors are values, handle explicitly.

## References

- [thiserror Documentation](https://docs.rs/thiserror/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling in Rust Blog Post](https://nick.groenen.me/posts/rust-error-handling/)
