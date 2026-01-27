//! Discord event handler implementation.
//!
//! This module provides the [`BotHandler`] which implements Serenity's
//! [`EventHandler`] trait to process Discord events.
//!
//! # Message Flow
//!
//! 1. Discord sends a message event
//! 2. Handler filters (ignores bots, empty messages)
//! 3. Message converted to internal format
//! 4. Sent to Claude API for response
//! 5. Reply posted back to Discord
//!
//! # Example
//!
//! ```
//! use ferrisbot::discord::handler::BotHandler;
//! use ferrisbot::llm::claude::ClaudeClient;
//! use std::sync::Arc;
//!
//! let claude = Arc::new(ClaudeClient::new("api-key"));
//! let handler = BotHandler::new(claude);
//! ```

use crate::discord::convert::{should_process_message, to_ferris_message};
use crate::llm::claude::ClaudeClient;
use crate::llm::types::{CreateMessageRequest, Message as ClaudeMessage};
use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;
use std::sync::Arc;
use tracing::{error, info};

/// Discord event handler with Claude integration.
///
/// The `BotHandler` processes Discord events, primarily messages,
/// and uses the Claude API to generate intelligent responses.
///
/// # Thread Safety
///
/// This handler is thread-safe and can be shared across multiple
/// shards (connections to Discord).
///
/// # Example
///
/// ```
/// use ferrisbot::discord::handler::BotHandler;
/// use ferrisbot::llm::claude::ClaudeClient;
/// use std::sync::Arc;
///
/// let claude = Arc::new(ClaudeClient::new("api-key"));
/// let handler = BotHandler::new(claude);
/// ```
pub struct BotHandler {
    claude_client: Arc<ClaudeClient>,
}

impl BotHandler {
    /// Create a new event handler with the given Claude client.
    ///
    /// # Arguments
    ///
    /// * `claude_client` - Shared Claude client for generating responses
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::discord::handler::BotHandler;
    /// use ferrisbot::llm::claude::ClaudeClient;
    /// use std::sync::Arc;
    ///
    /// let claude = Arc::new(ClaudeClient::new("api-key"));
    /// let handler = BotHandler::new(claude);
    /// ```
    pub fn new(claude_client: Arc<ClaudeClient>) -> Self {
        Self { claude_client }
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    /// Handle incoming messages.
    ///
    /// This method:
    /// 1. Filters messages that shouldn't be processed
    /// 2. Converts the message to internal format
    /// 3. Sends to Claude for a response
    /// 4. Replies with Claude's response (or an error message)
    async fn message(&self, ctx: Context, msg: Message) {
        // Filter out messages we shouldn't process
        if !should_process_message(&msg) {
            return;
        }

        // Convert to our internal message type
        let ferris_msg = to_ferris_message(&msg);

        // Send to Claude
        let request = CreateMessageRequest::new(vec![ClaudeMessage::user(&ferris_msg.content)]);

        match self.claude_client.send_message(request).await {
            Ok(response) => {
                let reply_text = response.get_text();
                if let Err(e) = msg.reply(&ctx, reply_text).await {
                    error!("Failed to send reply: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to get Claude response: {}", e);
                if let Err(e) = msg
                    .reply(
                        &ctx,
                        "Sorry, I encountered an error processing your message.",
                    )
                    .await
                {
                    error!("Failed to send error message: {}", e);
                }
            }
        }
    }

    /// Handle the ready event when the bot connects.
    ///
    /// Logs a message indicating the bot is online and ready.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected and ready!", ready.user.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let claude_client = Arc::new(ClaudeClient::new("test-key"));
        let handler = BotHandler::new(claude_client);
        let _ = handler;
    }

    #[tokio::test]
    async fn test_message_flow_with_mock() {
        // Integration tested in tests/full_flow.rs
    }
}
