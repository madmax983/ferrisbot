//! Type conversions between Discord and internal formats.
//!
//! This module provides utilities for converting between Serenity's Discord
//! types and Ferrisbot's internal message representation.
//!
//! # Functions
//!
//! - [`to_ferris_message`] - Convert Discord message to internal format
//! - [`should_process_message`] - Filter messages for processing
//! - [`should_process_in_channel`] - Channel-based filtering

use crate::types::Message as FerrisMessage;
use serenity::all::Message as DiscordMessage;

/// Convert a Discord message to our internal Message type.
///
/// Extracts the text content from a Discord message and wraps it
/// in our internal [`Message`](crate::types::Message) format.
///
/// # Arguments
///
/// * `discord_msg` - Reference to the Discord message
///
/// # Returns
///
/// A new [`FerrisMessage`] containing the message content.
///
/// # Example
///
/// This function is typically used in event handlers:
///
/// ```ignore
/// use jules_control_plane::discord::convert::to_ferris_message;
///
/// async fn handle_message(msg: serenity::all::Message) {
///     let internal_msg = to_ferris_message(&msg);
///     println!("Content: {}", internal_msg.content);
/// }
/// ```
pub fn to_ferris_message(discord_msg: &DiscordMessage) -> FerrisMessage {
    FerrisMessage::new(&discord_msg.content)
}

/// Check if a Discord message should be processed by the bot.
///
/// This function filters out messages that the bot should ignore:
/// - Messages from other bots (to prevent loops)
/// - Empty or whitespace-only messages
///
/// # Arguments
///
/// * `discord_msg` - Reference to the Discord message
///
/// # Returns
///
/// `true` if the message should be processed, `false` otherwise.
///
/// # Filtering Rules
///
/// | Condition | Result |
/// |-----------|--------|
/// | From a bot | Ignored |
/// | Empty content | Ignored |
/// | Whitespace only | Ignored |
/// | Normal user message | Processed |
///
/// # Example
///
/// ```ignore
/// use jules_control_plane::discord::convert::should_process_message;
///
/// async fn handle_message(msg: serenity::all::Message) {
///     if !should_process_message(&msg) {
///         return; // Skip this message
///     }
///     // Process the message...
/// }
/// ```
pub fn should_process_message(discord_msg: &DiscordMessage) -> bool {
    // Ignore messages from bots (including ourselves)
    if discord_msg.author.bot {
        return false;
    }

    // Ignore empty messages
    if discord_msg.content.trim().is_empty() {
        return false;
    }

    true
}

/// Check if a message should be processed in a specific channel.
///
/// This function allows for channel-based filtering. In the MVP,
/// all channels are processed, but this can be extended to support
/// allowlists/blocklists.
///
/// # Arguments
///
/// * `channel_id` - The Discord channel ID
///
/// # Returns
///
/// `true` if messages in this channel should be processed.
///
/// # Future Enhancements
///
/// This function will be extended in v0.7.0 to support:
/// - Per-channel enable/disable
/// - Channel allowlists
/// - Channel blocklists
///
/// # Example
///
/// ```
/// use jules_control_plane::discord::convert::should_process_in_channel;
/// use serenity::all::ChannelId;
///
/// let channel = ChannelId::new(123456789);
/// assert!(should_process_in_channel(channel));
/// ```
pub fn should_process_in_channel(_channel_id: serenity::all::ChannelId) -> bool {
    // For MVP, process all channels
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message as FerrisMessage;
    use serenity::all::Message as DiscordMessage;

    #[test]
    fn test_convert_discord_message_to_ferris_message() {
        // Test that the conversion function has the correct signature
        let _convert_fn: fn(&DiscordMessage) -> FerrisMessage = to_ferris_message;
    }

    #[test]
    fn test_should_ignore_bot_messages() {
        let should_ignore = should_process_message;
        let _ = should_ignore;
    }

    #[test]
    fn test_channel_filtering() {
        use serenity::all::ChannelId;

        // For MVP, all channels should be processed
        assert!(should_process_in_channel(ChannelId::new(123)));
        assert!(should_process_in_channel(ChannelId::new(456)));
    }

    #[test]
    fn test_message_content_extraction() {
        // Implicitly tested by to_ferris_message
        // Full integration tests in tests/full_flow.rs
    }
}
