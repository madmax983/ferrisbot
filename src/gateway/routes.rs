//! HTTP route handlers for the gateway.
//!
//! This module contains the handler functions for HTTP endpoints.
//!
//! # Endpoints
//!
//! | Path | Handler | Description |
//! |------|---------|-------------|
//! | `/health` | [`health`] | Health check endpoint |

/// Health check endpoint handler.
///
/// Returns "OK" to indicate the server is running and healthy.
/// This endpoint is used by load balancers and monitoring systems.
///
/// # Response
///
/// Returns a plain text "OK" with status code 200.
///
/// # Example
///
/// Using curl:
///
/// ```bash
/// $ curl http://localhost:18789/health
/// OK
/// ```
///
/// # In Tests
///
/// ```
/// use ferrisbot::gateway::routes::health;
///
/// #[tokio::main]
/// async fn main() {
///     let response = health().await;
///     assert_eq!(response, "OK");
/// }
/// ```
pub async fn health() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_returns_ok() {
        let response = health().await;
        assert_eq!(response, "OK");
    }
}
