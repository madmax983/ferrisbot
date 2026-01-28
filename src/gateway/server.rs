//! Axum server setup and configuration.
//!
//! This module provides functions for creating the HTTP router and
//! configuring the server address.
//!
//! # Example
//!
//! ```no_run
//! use jules_control_plane::gateway::server::{create_router, create_addr, DEFAULT_PORT};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let router = create_router();
//!     let addr = create_addr(DEFAULT_PORT);
//!
//!     println!("Starting server on {}", addr);
//!
//!     let listener = tokio::net::TcpListener::bind(addr).await?;
//!     axum::serve(listener, router).await
//! }
//! ```

use axum::{routing::get, Router};
use std::net::SocketAddr;

/// Default port for the gateway server.
///
/// The gateway listens on port 18789 by default. This can be overridden
/// via the `GATEWAY_PORT` environment variable.
///
/// # Example
///
/// ```
/// use jules_control_plane::gateway::server::DEFAULT_PORT;
///
/// assert_eq!(DEFAULT_PORT, 18789);
/// ```
pub const DEFAULT_PORT: u16 = 18789;

/// Create the gateway router with all routes configured.
///
/// The router includes:
/// - `GET /health` - Health check endpoint
/// - `GET /ws` - WebSocket upgrade endpoint
///
/// # Example
///
/// ```
/// use jules_control_plane::gateway::server::create_router;
///
/// let router = create_router();
/// // Router is ready to be served
/// ```
///
/// # With Axum
///
/// ```no_run
/// use jules_control_plane::gateway::server::create_router;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let router = create_router();
///     let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
///     axum::serve(listener, router).await
/// }
/// ```
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(super::routes::health))
        .route("/ws", get(super::ws::ws_handler))
}

/// Create a socket address for the given port.
///
/// Creates an address bound to localhost (127.0.0.1) on the specified port.
///
/// # Arguments
///
/// * `port` - The port number to bind to
///
/// # Example
///
/// ```
/// use jules_control_plane::gateway::server::create_addr;
///
/// let addr = create_addr(8080);
/// assert_eq!(addr.port(), 8080);
/// assert_eq!(addr.ip().to_string(), "127.0.0.1");
/// ```
///
/// # With Default Port
///
/// ```
/// use jules_control_plane::gateway::server::{create_addr, DEFAULT_PORT};
///
/// let addr = create_addr(DEFAULT_PORT);
/// assert_eq!(addr.port(), 18789);
/// ```
pub fn create_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_router() {
        let router = create_router();
        // Just verify it compiles and creates
        let _ = router;
    }

    #[test]
    fn test_create_addr() {
        let addr = create_addr(8080);
        assert_eq!(addr.port(), 8080);
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
    }

    #[tokio::test]
    async fn test_health_endpoint_via_router() {
        // Full integration test in tests/full_flow.rs
        let router = create_router();
        let _ = router;
    }
}
