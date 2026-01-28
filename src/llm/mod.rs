//! Language model integration for Ferrisbot.
//!
//! This module provides clients for interacting with large language models,
//! currently supporting Anthropic's Claude API.
//!
//! # Modules
//!
//! - [`claude`] - Claude API client implementation
//! - [`types`] - Request and response types for the API
//!
//! # Example
//!
//! ```no_run
//! use jules_control_plane::llm::claude::ClaudeClient;
//! use jules_control_plane::llm::types::{CreateMessageRequest, Message};
//!
//! #[tokio::main]
//! async fn main() -> jules_control_plane::error::Result<()> {
//!     // Create a client
//!     let client = ClaudeClient::new("your-api-key");
//!
//!     // Build a request
//!     let request = CreateMessageRequest::new(vec![
//!         Message::user("Hello, Claude!")
//!     ]);
//!
//!     // Send the request
//!     let response = client.send_message(request).await?;
//!
//!     println!("Claude says: {}", response.get_text());
//!     Ok(())
//! }
//! ```
//!
//! # Architecture
//!
//! The LLM module is designed for extensibility. While currently only
//! Claude is supported, the structure allows for adding other providers
//! in the future (e.g., OpenAI, local models).

pub mod claude;
pub mod types;

#[cfg(test)]
mod tests {
    use super::claude::ClaudeClient;

    #[test]
    fn test_claude_client_creation() {
        let api_key = "test-api-key";
        let _client = ClaudeClient::new(api_key);
    }

    #[test]
    fn test_claude_client_stores_api_key() {
        let api_key = "test-api-key-123";
        let client = ClaudeClient::new(api_key);
        assert_eq!(client.api_key(), api_key);
    }
}
