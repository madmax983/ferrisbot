//! Discord bot integration for Ferrisbot.
//!
//! This module provides the Discord bot functionality using the
//! [Serenity](https://docs.rs/serenity/) framework. It handles:
//!
//! - Bot connection and lifecycle management
//! - Message event handling
//! - Integration with the Claude API for responses
//! - Type conversions between Discord and internal formats
//!
//! # Modules
//!
//! - [`bot`] - Discord bot wrapper and configuration
//! - [`handler`] - Event handler implementation
//! - [`convert`] - Type conversion utilities
//!
//! # Example
//!
//! ```no_run
//! use ferrisbot::discord::bot::DiscordBot;
//! use ferrisbot::llm::claude::ClaudeClient;
//! use std::sync::Arc;
//! use tokio::sync::mpsc;
//!
//! #[tokio::main]
//! async fn main() -> ferrisbot::error::Result<()> {
//!     let bot = DiscordBot::new("your-discord-token");
//!     let claude = Arc::new(ClaudeClient::new("your-api-key"));
//!     let (_, shutdown_rx) = mpsc::channel(1);
//!
//!     bot.start(claude, shutdown_rx).await
//! }
//! ```
//!
//! # Architecture
//!
//! The Discord module follows a layered design:
//!
//! 1. **Bot** - Manages the Discord client connection
//! 2. **Handler** - Processes incoming events (messages, ready)
//! 3. **Convert** - Transforms types between Discord and internal formats

pub mod bot;
pub mod convert;
pub mod handler;

#[cfg(test)]
mod tests {
    use super::bot::DiscordBot;

    #[test]
    fn test_discord_bot_creation() {
        let token = "test-token";
        let _bot = DiscordBot::new(token);
    }

    #[test]
    fn test_bot_stores_token() {
        let token = "test-token-123";
        let bot = DiscordBot::new(token);
        assert_eq!(bot.token(), token);
    }
}
