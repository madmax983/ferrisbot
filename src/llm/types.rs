//! Claude API request and response types.
//!
//! This module defines the data structures for communicating with the
//! Anthropic Claude API. All types implement `Serialize` and `Deserialize`
//! for JSON encoding/decoding.
//!
//! # Example
//!
//! ```
//! use ferrisbot::llm::types::{CreateMessageRequest, Message};
//!
//! // Create a simple request
//! let request = CreateMessageRequest::new(vec![
//!     Message::user("Hello, Claude!")
//! ]);
//!
//! // Serialize to JSON
//! let json = serde_json::to_string(&request).unwrap();
//! assert!(json.contains("Hello, Claude!"));
//! ```

use serde::{Deserialize, Serialize};

/// Default model to use for Claude API requests.
///
/// This is updated to match the latest available Claude model.
pub const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";

/// Default maximum tokens for Claude API responses.
pub const DEFAULT_MAX_TOKENS: u32 = 4096;

/// Role identifier for user messages.
pub const ROLE_USER: &str = "user";

/// Role identifier for assistant (Claude) messages.
pub const ROLE_ASSISTANT: &str = "assistant";

/// A message in a Claude API conversation.
///
/// Messages represent individual turns in a conversation, with a role
/// indicating who sent the message (user or assistant).
///
/// # Example
///
/// ```
/// use ferrisbot::llm::types::Message;
///
/// // Create a user message
/// let user_msg = Message::user("What is 2 + 2?");
/// assert_eq!(user_msg.role, "user");
///
/// // Create an assistant message
/// let assistant_msg = Message::assistant("2 + 2 = 4");
/// assert_eq!(assistant_msg.role, "assistant");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender ("user" or "assistant").
    pub role: String,

    /// The text content of the message.
    pub content: String,
}

impl Message {
    /// Create a new user message.
    ///
    /// # Arguments
    ///
    /// * `content` - The message text
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::Message;
    ///
    /// let msg = Message::user("Hello!");
    /// assert_eq!(msg.role, "user");
    /// assert_eq!(msg.content, "Hello!");
    /// ```
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ROLE_USER.to_string(),
            content: content.into(),
        }
    }

    /// Create a new assistant message.
    ///
    /// Used when including previous assistant responses in a conversation.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::Message;
    ///
    /// let msg = Message::assistant("I can help with that!");
    /// assert_eq!(msg.role, "assistant");
    /// ```
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ROLE_ASSISTANT.to_string(),
            content: content.into(),
        }
    }
}

/// Request to create a message via the Claude API.
///
/// This struct represents the request body sent to the Claude API's
/// `/v1/messages` endpoint.
///
/// # Example
///
/// ```
/// use ferrisbot::llm::types::{CreateMessageRequest, Message};
///
/// // Simple request with defaults
/// let request = CreateMessageRequest::new(vec![
///     Message::user("Hello!")
/// ]);
///
/// // Custom model and token limit
/// let request = CreateMessageRequest::new(vec![
///     Message::user("Write a poem")
/// ])
/// .with_model("claude-opus-4-20250514")
/// .with_max_tokens(2048);
/// ```
///
/// # JSON Format
///
/// ```
/// use ferrisbot::llm::types::{CreateMessageRequest, Message};
///
/// let request = CreateMessageRequest::new(vec![Message::user("Hi")]);
/// let json = serde_json::to_value(&request).unwrap();
///
/// assert_eq!(json["model"], "claude-sonnet-4-20250514");
/// assert_eq!(json["max_tokens"], 4096);
/// assert_eq!(json["messages"][0]["role"], "user");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// The model to use (e.g., "claude-sonnet-4-20250514").
    pub model: String,

    /// Maximum tokens to generate in the response.
    pub max_tokens: u32,

    /// The conversation messages.
    pub messages: Vec<Message>,
}

impl CreateMessageRequest {
    /// Create a new request with default settings.
    ///
    /// Uses [`DEFAULT_MODEL`] and [`DEFAULT_MAX_TOKENS`].
    ///
    /// # Arguments
    ///
    /// * `messages` - The conversation messages
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::{CreateMessageRequest, Message, DEFAULT_MODEL};
    ///
    /// let request = CreateMessageRequest::new(vec![Message::user("Hello")]);
    /// assert_eq!(request.model, DEFAULT_MODEL);
    /// ```
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            model: DEFAULT_MODEL.to_string(),
            max_tokens: DEFAULT_MAX_TOKENS,
            messages,
        }
    }

    /// Set a custom model.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::{CreateMessageRequest, Message};
    ///
    /// let request = CreateMessageRequest::new(vec![Message::user("Hi")])
    ///     .with_model("claude-opus-4-20250514");
    ///
    /// assert_eq!(request.model, "claude-opus-4-20250514");
    /// ```
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set the maximum tokens for the response.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::{CreateMessageRequest, Message};
    ///
    /// let request = CreateMessageRequest::new(vec![Message::user("Hi")])
    ///     .with_max_tokens(1024);
    ///
    /// assert_eq!(request.max_tokens, 1024);
    /// ```
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

/// A content block in a Claude API response.
///
/// Claude responses contain an array of content blocks, each with a type
/// and associated data. Currently, only "text" blocks are common.
///
/// # Example
///
/// ```
/// use ferrisbot::llm::types::ContentBlock;
///
/// let json = r#"{"type": "text", "text": "Hello!"}"#;
/// let block: ContentBlock = serde_json::from_str(json).unwrap();
///
/// assert_eq!(block.content_type, "text");
/// assert_eq!(block.text, "Hello!");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    /// The type of content block (typically "text").
    #[serde(rename = "type")]
    pub content_type: String,

    /// The text content of the block.
    pub text: String,
}

/// Token usage statistics from a Claude API response.
///
/// # Example
///
/// ```
/// use ferrisbot::llm::types::Usage;
///
/// let json = r#"{"input_tokens": 10, "output_tokens": 25}"#;
/// let usage: Usage = serde_json::from_str(json).unwrap();
///
/// assert_eq!(usage.input_tokens, 10);
/// assert_eq!(usage.output_tokens, 25);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the input (prompt).
    pub input_tokens: u32,

    /// Number of tokens in the output (response).
    pub output_tokens: u32,
}

impl Usage {
    /// Get the total token count.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::Usage;
    ///
    /// let usage = Usage { input_tokens: 10, output_tokens: 25 };
    /// assert_eq!(usage.total(), 35);
    /// ```
    pub fn total(&self) -> u32 {
        self.input_tokens + self.output_tokens
    }
}

/// Response from the Claude API.
///
/// This struct represents the full response from the Claude API's
/// `/v1/messages` endpoint.
///
/// # Example
///
/// ```
/// use ferrisbot::llm::types::MessageResponse;
///
/// let json = r#"{
///     "id": "msg_123",
///     "type": "message",
///     "role": "assistant",
///     "content": [{"type": "text", "text": "Hello!"}],
///     "model": "claude-sonnet-4-20250514",
///     "stop_reason": "end_turn",
///     "usage": {"input_tokens": 5, "output_tokens": 3}
/// }"#;
///
/// let response: MessageResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(response.get_text(), "Hello!");
/// assert_eq!(response.total_tokens(), 8);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Unique identifier for this message.
    pub id: String,

    /// The type of response (always "message").
    #[serde(rename = "type")]
    pub message_type: String,

    /// The role of the responder (always "assistant").
    pub role: String,

    /// The content blocks in the response.
    pub content: Vec<ContentBlock>,

    /// The model that generated the response.
    pub model: String,

    /// The reason the model stopped generating.
    pub stop_reason: String,

    /// Token usage statistics.
    pub usage: Usage,
}

impl MessageResponse {
    /// Extract the text content from the response.
    ///
    /// Concatenates all text blocks into a single string.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::MessageResponse;
    ///
    /// let json = r#"{
    ///     "id": "msg_1", "type": "message", "role": "assistant",
    ///     "content": [{"type": "text", "text": "Part 1"}, {"type": "text", "text": "Part 2"}],
    ///     "model": "claude-sonnet-4-20250514", "stop_reason": "end_turn",
    ///     "usage": {"input_tokens": 5, "output_tokens": 10}
    /// }"#;
    ///
    /// let response: MessageResponse = serde_json::from_str(json).unwrap();
    /// assert_eq!(response.get_text(), "Part 1Part 2");
    /// ```
    pub fn get_text(&self) -> String {
        // Fast path for single block (very common case)
        if self.content.len() == 1 {
            if let Some(block) = self.content.first() {
                if block.content_type == "text" {
                    return block.text.clone();
                }
            }
        }

        // Calculate capacity to avoid reallocations
        let capacity: usize = self
            .content
            .iter()
            .filter(|block| block.content_type == "text")
            .map(|block| block.text.len())
            .sum();

        let mut result = String::with_capacity(capacity);
        for block in self
            .content
            .iter()
            .filter(|block| block.content_type == "text")
        {
            result.push_str(&block.text);
        }
        result
    }

    /// Check if the response has any content.
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::MessageResponse;
    ///
    /// let json = r#"{
    ///     "id": "msg_1", "type": "message", "role": "assistant",
    ///     "content": [],
    ///     "model": "claude-sonnet-4-20250514", "stop_reason": "end_turn",
    ///     "usage": {"input_tokens": 5, "output_tokens": 0}
    /// }"#;
    ///
    /// let response: MessageResponse = serde_json::from_str(json).unwrap();
    /// assert!(!response.has_content());
    /// ```
    pub fn has_content(&self) -> bool {
        !self.content.is_empty()
    }

    /// Get the total token count (input + output).
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::llm::types::MessageResponse;
    ///
    /// let json = r#"{
    ///     "id": "msg_1", "type": "message", "role": "assistant",
    ///     "content": [{"type": "text", "text": "Hi"}],
    ///     "model": "claude-sonnet-4-20250514", "stop_reason": "end_turn",
    ///     "usage": {"input_tokens": 10, "output_tokens": 5}
    /// }"#;
    ///
    /// let response: MessageResponse = serde_json::from_str(json).unwrap();
    /// assert_eq!(response.total_tokens(), 15);
    /// ```
    pub fn total_tokens(&self) -> u32 {
        self.usage.input_tokens + self.usage.output_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message_request() {
        let request = CreateMessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            }],
        };

        let json = serde_json::to_value(&request).expect("should serialize");
        assert_eq!(json["model"], "claude-sonnet-4-20250514");
        assert_eq!(json["max_tokens"], 1024);
        assert_eq!(json["messages"][0]["role"], "user");
        assert_eq!(json["messages"][0]["content"], "Hello!");
    }

    #[test]
    fn test_message_role() {
        let msg = Message {
            role: "assistant".to_string(),
            content: "Hi there!".to_string(),
        };

        let json = serde_json::to_value(&msg).expect("should serialize");
        assert_eq!(json["role"], "assistant");
        assert_eq!(json["content"], "Hi there!");
    }

    #[test]
    fn test_message_builders() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, ROLE_USER);
        assert_eq!(user_msg.content, "Hello");

        let assistant_msg = Message::assistant("Hi!");
        assert_eq!(assistant_msg.role, ROLE_ASSISTANT);
        assert_eq!(assistant_msg.content, "Hi!");
    }

    #[test]
    fn test_request_builder() {
        let request = CreateMessageRequest::new(vec![Message::user("Test")])
            .with_model("claude-opus-4-20250514")
            .with_max_tokens(2048);

        assert_eq!(request.model, "claude-opus-4-20250514");
        assert_eq!(request.max_tokens, 2048);
        assert_eq!(request.messages.len(), 1);
    }

    #[test]
    fn test_request_defaults() {
        let request = CreateMessageRequest::new(vec![Message::user("Test")]);
        assert_eq!(request.model, DEFAULT_MODEL);
        assert_eq!(request.max_tokens, DEFAULT_MAX_TOKENS);
    }

    #[test]
    fn test_request_serialization_format() {
        let request = CreateMessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 2048,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Test".to_string(),
            }],
        };

        let json = serde_json::to_string(&request).expect("should serialize");
        assert!(json.contains(r#""model":"claude-sonnet-4-20250514""#));
        assert!(json.contains(r#""max_tokens":2048"#));
        assert!(json.contains(r#""messages":"#));
    }

    #[test]
    fn test_parse_message_response() {
        let json = r#"{
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "Hello! How can I help you?"
                }
            ],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 10,
                "output_tokens": 20
            }
        }"#;

        let response: MessageResponse = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(response.id, "msg_123");
        assert_eq!(response.role, "assistant");
        assert_eq!(response.model, "claude-sonnet-4-20250514");
    }

    #[test]
    fn test_extract_text_from_response() {
        let json = r#"{
            "id": "msg_456",
            "type": "message",
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "Test response"
                }
            ],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 5,
                "output_tokens": 10
            }
        }"#;

        let response: MessageResponse = serde_json::from_str(json).expect("should deserialize");
        let text = response.get_text();
        assert_eq!(text, "Test response");
    }

    #[test]
    fn test_response_empty_content() {
        let json = r#"{
            "id": "msg_789",
            "type": "message",
            "role": "assistant",
            "content": [],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 5,
                "output_tokens": 0
            }
        }"#;

        let response: MessageResponse = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(response.get_text(), "");
        assert!(!response.has_content());
    }

    #[test]
    fn test_response_token_count() {
        let json = r#"{
            "id": "msg_101",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "Hi"}],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 15,
                "output_tokens": 25
            }
        }"#;

        let response: MessageResponse = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(response.total_tokens(), 40);
        assert!(response.has_content());
    }

    #[test]
    fn test_usage_total() {
        let usage = Usage {
            input_tokens: 10,
            output_tokens: 20,
        };
        assert_eq!(usage.total(), 30);
    }

    #[test]
    fn test_extract_text_from_multiple_blocks() {
        let json = r#"{
            "id": "msg_multi",
            "type": "message",
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "Part 1"
                },
                {
                    "type": "image",
                    "text": "ignored"
                },
                {
                    "type": "text",
                    "text": "Part 2"
                }
            ],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 5,
                "output_tokens": 10
            }
        }"#;

        let response: MessageResponse = serde_json::from_str(json).expect("should deserialize");
        let text = response.get_text();
        assert_eq!(text, "Part 1Part 2");
    }
}
