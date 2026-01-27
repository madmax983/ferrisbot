//! HTTP and WebSocket gateway for Ferrisbot.
//!
//! This module provides the HTTP gateway server using [Axum](https://docs.rs/axum/),
//! offering REST endpoints and WebSocket connections for monitoring and control.
//!
//! # Endpoints
//!
//! | Path | Method | Description |
//! |------|--------|-------------|
//! | `/health` | GET | Health check endpoint |
//! | `/ws` | GET | WebSocket upgrade for monitoring |
//!
//! # Modules
//!
//! - [`server`] - Axum server setup and configuration
//! - [`routes`] - HTTP route handlers
//! - [`ws`] - WebSocket handler and protocol
//!
//! # Example
//!
//! ```no_run
//! use ferrisbot::gateway::server::{create_router, create_addr, DEFAULT_PORT};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let router = create_router();
//!     let addr = create_addr(DEFAULT_PORT);
//!
//!     let listener = tokio::net::TcpListener::bind(addr).await?;
//!     axum::serve(listener, router).await
//! }
//! ```
//!
//! # WebSocket Protocol
//!
//! The WebSocket endpoint uses a JSON protocol with tagged unions:
//!
//! ```json
//! // Request
//! {"type": "ping"}
//! {"type": "status"}
//!
//! // Response
//! {"type": "pong"}
//! {"type": "status", "bot_connected": true}
//! {"type": "error", "message": "..."}
//! ```

pub mod routes;
pub mod server;
pub mod ws;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_health_endpoint() {
        use super::routes::health;

        let response = health().await;
        assert_eq!(response, "OK");
    }
}
