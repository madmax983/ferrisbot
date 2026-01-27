//! Discord bot wrapper and configuration.
//!
//! This module provides the [`DiscordBot`] struct which wraps the Serenity
//! Discord client with configuration and lifecycle management.
//!
//! # Example
//!
//! ```
//! use ferrisbot::discord::bot::DiscordBot;
//! use serenity::all::GatewayIntents;
//!
//! // Simple creation with default intents
//! let bot = DiscordBot::new("token");
//!
//! // Custom intents via builder
//! let bot = DiscordBot::builder("token")
//!     .intents(GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES)
//!     .build();
//! ```

use crate::error::Result;
use crate::llm::claude::ClaudeClient;
use serenity::all::{Client, GatewayIntents};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Discord bot wrapper with configuration.
///
/// The `DiscordBot` struct encapsulates Discord client configuration
/// and provides methods for starting and managing the bot lifecycle.
///
/// # Default Intents
///
/// By default, the bot requests:
/// - `GUILD_MESSAGES` - To receive messages in server channels
/// - `MESSAGE_CONTENT` - To read message content (required for responding)
///
/// # Example
///
/// ```
/// use ferrisbot::discord::bot::DiscordBot;
///
/// let bot = DiscordBot::new("your-token");
/// assert!(bot.intents().contains(serenity::all::GatewayIntents::GUILD_MESSAGES));
/// ```
pub struct DiscordBot {
    token: String,
    intents: GatewayIntents,
}

impl DiscordBot {
    /// Create a new Discord bot with the given token and default intents.
    ///
    /// # Arguments
    ///
    /// * `token` - Discord bot token from the Developer Portal
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::bot::DiscordBot;
    ///
    /// let bot = DiscordBot::new("your-bot-token");
    /// ```
    pub fn new(token: impl Into<String>) -> Self {
        Self::builder(token).build()
    }

    /// Create a builder for configuring the bot.
    ///
    /// Use the builder to customize intents or other settings.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::bot::DiscordBot;
    /// use serenity::all::GatewayIntents;
    ///
    /// let bot = DiscordBot::builder("token")
    ///     .intents(GatewayIntents::all())
    ///     .build();
    /// ```
    pub fn builder(token: impl Into<String>) -> DiscordBotBuilder {
        DiscordBotBuilder {
            token: token.into(),
            intents: GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
        }
    }

    /// Get the bot token.
    ///
    /// Primarily used for testing.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::bot::DiscordBot;
    ///
    /// let bot = DiscordBot::new("my-token");
    /// assert_eq!(bot.token(), "my-token");
    /// ```
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Get the configured gateway intents.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::bot::DiscordBot;
    /// use serenity::all::GatewayIntents;
    ///
    /// let bot = DiscordBot::new("token");
    /// let intents = bot.intents();
    ///
    /// assert!(intents.contains(GatewayIntents::GUILD_MESSAGES));
    /// assert!(intents.contains(GatewayIntents::MESSAGE_CONTENT));
    /// ```
    pub fn intents(&self) -> GatewayIntents {
        self.intents
    }

    /// Start the bot with Claude integration.
    ///
    /// This method starts the Discord client and begins processing events.
    /// It will run until a shutdown signal is received via the provided channel.
    ///
    /// # Arguments
    ///
    /// * `claude_client` - Shared Claude client for generating responses
    /// * `shutdown_rx` - Channel receiver for shutdown signals
    ///
    /// # Errors
    ///
    /// Returns an error if the Discord client fails to initialize.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ferrisbot::discord::bot::DiscordBot;
    /// use ferrisbot::llm::claude::ClaudeClient;
    /// use std::sync::Arc;
    /// use tokio::sync::mpsc;
    ///
    /// #[tokio::main]
    /// async fn main() -> ferrisbot::error::Result<()> {
    ///     let bot = DiscordBot::new("token");
    ///     let claude = Arc::new(ClaudeClient::new("api-key"));
    ///     let (tx, rx) = mpsc::channel(1);
    ///
    ///     // Start bot (runs until shutdown signal)
    ///     bot.start(claude, rx).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(
        &self,
        claude_client: Arc<ClaudeClient>,
        mut shutdown_rx: mpsc::Receiver<()>,
    ) -> Result<()> {
        let handler = crate::discord::handler::BotHandler::new(claude_client);

        let mut client = Client::builder(&self.token, self.intents)
            .event_handler(handler)
            .await
            .map_err(|e| {
                crate::error::FerrisError::Discord(format!("Failed to create client: {}", e))
            })?;

        // Spawn client in background
        let shard_manager = client.shard_manager.clone();

        tokio::spawn(async move {
            if let Err(e) = client.start().await {
                tracing::error!("Discord client error: {}", e);
            }
        });

        // Wait for shutdown signal
        shutdown_rx.recv().await;

        // Shutdown cleanly
        shard_manager.shutdown_all().await;

        Ok(())
    }
}

/// Builder for configuring a [`DiscordBot`].
///
/// # Example
///
/// ```
/// use ferrisbot::discord::bot::DiscordBot;
/// use serenity::all::GatewayIntents;
///
/// let bot = DiscordBot::builder("token")
///     .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
///     .build();
/// ```
pub struct DiscordBotBuilder {
    token: String,
    intents: GatewayIntents,
}

impl DiscordBotBuilder {
    /// Set custom gateway intents.
    ///
    /// # Arguments
    ///
    /// * `intents` - The gateway intents to request
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::bot::DiscordBot;
    /// use serenity::all::GatewayIntents;
    ///
    /// let bot = DiscordBot::builder("token")
    ///     .intents(GatewayIntents::non_privileged())
    ///     .build();
    /// ```
    pub fn intents(mut self, intents: GatewayIntents) -> Self {
        self.intents = intents;
        self
    }

    /// Build the [`DiscordBot`].
    pub fn build(self) -> DiscordBot {
        DiscordBot {
            token: self.token,
            intents: self.intents,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_has_intents() {
        let bot = DiscordBot::new("test-token");
        let intents = bot.intents();
        assert!(intents.contains(GatewayIntents::GUILD_MESSAGES));
        assert!(intents.contains(GatewayIntents::MESSAGE_CONTENT));
    }

    #[test]
    fn test_bot_builder() {
        let custom_intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
        let bot = DiscordBot::builder("test-token")
            .intents(custom_intents)
            .build();

        assert_eq!(bot.token(), "test-token");
        assert_eq!(bot.intents(), custom_intents);
    }
}
