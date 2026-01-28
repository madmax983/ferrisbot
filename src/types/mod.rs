//! Core domain types for Ferrisbot.
//!
//! This module contains the fundamental data structures used throughout
//! the application for representing messages and other domain concepts.
//!
//! # Example
//!
//! ```
//! use jules_control_plane::types::Message;
//!
//! // Create a new message
//! let msg = Message::new("Hello, world!");
//! assert_eq!(msg.content, "Hello, world!");
//!
//! // Messages can be serialized to JSON
//! let json = serde_json::to_string(&msg).unwrap();
//! assert!(json.contains("Hello, world!"));
//! ```

use serde::{Deserialize, Serialize};

/// Represents a chat message that flows through the system.
///
/// This is the internal message representation used by Ferrisbot,
/// distinct from Discord's or Claude's message formats. It provides
/// a unified interface for message handling across different services.
///
/// # Fields
///
/// * `content` - The text content of the message
///
/// # Examples
///
/// Creating a new message:
///
/// ```
/// use jules_control_plane::types::Message;
///
/// let msg = Message::new("Hello!");
/// assert_eq!(msg.content, "Hello!");
/// ```
///
/// Creating from a `String`:
///
/// ```
/// use jules_control_plane::types::Message;
///
/// let content = String::from("Dynamic content");
/// let msg = Message::new(content);
/// assert_eq!(msg.content, "Dynamic content");
/// ```
///
/// Serialization:
///
/// ```
/// use jules_control_plane::types::Message;
///
/// let msg = Message::new("Test");
/// let json = serde_json::to_string(&msg).unwrap();
/// assert_eq!(json, r#"{"content":"Test"}"#);
/// ```
///
/// Deserialization:
///
/// ```
/// use jules_control_plane::types::Message;
///
/// let json = r#"{"content":"From JSON"}"#;
/// let msg: Message = serde_json::from_str(json).unwrap();
/// assert_eq!(msg.content, "From JSON");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// The text content of the message.
    pub content: String,
}

impl Message {
    /// Create a new message with the given content.
    ///
    /// This method accepts any type that implements `Into<String>`,
    /// allowing both `&str` and `String` arguments.
    ///
    /// # Arguments
    ///
    /// * `content` - The message content
    ///
    /// # Examples
    ///
    /// ```
    /// use jules_control_plane::types::Message;
    ///
    /// // From a string literal
    /// let msg1 = Message::new("Hello");
    ///
    /// // From a String
    /// let msg2 = Message::new(String::from("World"));
    ///
    /// // From a formatted string
    /// let name = "Ferris";
    /// let msg3 = Message::new(format!("Hello, {}!", name));
    /// assert_eq!(msg3.content, "Hello, Ferris!");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Check if the message is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use jules_control_plane::types::Message;
    ///
    /// let empty = Message::new("");
    /// assert!(empty.is_empty());
    ///
    /// let whitespace = Message::new("   ");
    /// assert!(!whitespace.is_empty()); // Contains spaces
    ///
    /// let content = Message::new("Hello");
    /// assert!(!content.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get the length of the message content in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use jules_control_plane::types::Message;
    ///
    /// let msg = Message::new("Hello");
    /// assert_eq!(msg.len(), 5);
    ///
    /// // UTF-8 characters may be more than 1 byte
    /// let emoji = Message::new("🦀");
    /// assert_eq!(emoji.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.content.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_exists() {
        let _msg: Message;
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::new("Hello, world!");

        let json = serde_json::to_string(&msg).expect("should serialize");
        assert!(json.contains("Hello, world!"));
    }

    #[test]
    fn test_message_new() {
        let msg = Message::new("test");
        assert_eq!(msg.content, "test");

        let msg2 = Message::new("test".to_string());
        assert_eq!(msg2.content, "test");
    }

    #[test]
    fn test_message_deserialization() {
        let json = r#"{"content":"Test message"}"#;
        let msg: Message = serde_json::from_str(json).expect("should deserialize");

        assert_eq!(msg.content, "Test message");
    }

    #[test]
    fn test_message_is_empty() {
        assert!(Message::new("").is_empty());
        assert!(!Message::new("x").is_empty());
    }

    #[test]
    fn test_message_len() {
        assert_eq!(Message::new("hello").len(), 5);
        assert_eq!(Message::new("").len(), 0);
    }
}
