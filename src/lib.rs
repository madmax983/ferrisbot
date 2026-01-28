//! # Ferrisbot
//!
//! A Discord bot powered by Claude AI, built with Rust.
//!
//! Ferrisbot provides a foundation for building intelligent Discord bots that leverage
//! Anthropic's Claude API for natural language understanding and generation.
//!
//! ## Features
//!
//! - **Discord Integration**: Full Discord bot support via [Serenity](https://docs.rs/serenity/)
//! - **Claude AI**: Integration with Anthropic's Claude API for intelligent responses
//! - **HTTP Gateway**: REST API and WebSocket support via [Axum](https://docs.rs/axum/)
//! - **Async Runtime**: Built on [Tokio](https://tokio.rs/) for high-performance async I/O
//!
//! ## Quick Start
//!
//! ```no_run
//! use jules_control_plane::app::{App, AppConfig};
//!
//! #[tokio::main]
//! async fn main() -> jules_control_plane::error::Result<()> {
//!     // Load configuration from environment variables
//!     let config = AppConfig::from_env()?;
//!
//!     // Create and run the application
//!     let app = App::new(config);
//!     app.run().await
//! }
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`app`] - Application orchestration and configuration
//! - [`discord`] - Discord bot integration
//! - [`llm`] - Language model clients (Claude API)
//! - [`gateway`] - HTTP/WebSocket gateway server
//! - [`types`] - Core domain types
//! - [`error`] - Error handling
//!
//! ## Environment Variables
//!
//! The following environment variables are required:
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `DISCORD_TOKEN` | Discord bot token |
//! | `ANTHROPIC_API_KEY` | Anthropic API key for Claude |
//!
//! Optional variables:
//!
//! | Variable | Description | Default |
//! |----------|-------------|---------|
//! | `GATEWAY_PORT` | HTTP gateway port | `18789` |
//! | `RUST_LOG` | Logging level | `info` |

pub mod app;
pub mod discord;
pub mod error;
pub mod gateway;
pub mod llm;
pub mod types;
