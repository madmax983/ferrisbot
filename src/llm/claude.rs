//! Claude API client implementation.
//!
//! This module provides a client for interacting with Anthropic's Claude API.
//! It handles authentication, request formatting, and response parsing.
//!
//! # Example
//!
//! ```no_run
//! use jules_control_plane::llm::claude::ClaudeClient;
//! use jules_control_plane::llm::types::{CreateMessageRequest, Message};
//!
//! #[tokio::main]
//! async fn main() -> jules_control_plane::error::Result<()> {
//!     let client = ClaudeClient::new("your-api-key");
//!
//!     let request = CreateMessageRequest::new(vec![
//!         Message::user("What is the capital of France?")
//!     ]);
//!
//!     let response = client.send_message(request).await?;
//!     println!("{}", response.get_text());
//!     Ok(())
//! }
//! ```
//!
//! # Builder Pattern
//!
//! For advanced configuration, use the builder:
//!
//! ```
//! use jules_control_plane::llm::claude::ClaudeClient;
//! use std::time::Duration;
//!
//! let client = ClaudeClient::builder("api-key")
//!     .api_url("https://custom.endpoint.com/v1/messages")
//!     .timeout(Duration::from_secs(60))
//!     .build();
//! ```

use crate::error::{FerrisError, Result};
use crate::llm::types::{CreateMessageRequest, MessageResponse};
use std::time::Duration;

/// Default URL for the Claude API messages endpoint.
const DEFAULT_API_URL: &str = "https://api.anthropic.com/v1/messages";

/// Default request timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Anthropic API version header value.
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// Client for interacting with Anthropic's Claude API.
///
/// The `ClaudeClient` handles HTTP communication with the Claude API,
/// including authentication via API key and proper header formatting.
///
/// # Example
///
/// ```
/// use jules_control_plane::llm::claude::ClaudeClient;
///
/// // Simple creation with defaults
/// let client = ClaudeClient::new("your-api-key");
/// assert_eq!(client.api_url(), "https://api.anthropic.com/v1/messages");
/// ```
///
/// # Thread Safety
///
/// `ClaudeClient` is `Send + Sync` and can be safely shared across threads
/// using `Arc<ClaudeClient>`.
pub struct ClaudeClient {
    api_key: String,
    api_url: String,
    http_client: reqwest::Client,
}

impl ClaudeClient {
    /// Create a new Claude API client with the given API key.
    ///
    /// Uses default settings:
    /// - API URL: `https://api.anthropic.com/v1/messages`
    /// - Timeout: 30 seconds
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Anthropic API key
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    ///
    /// let client = ClaudeClient::new("sk-ant-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::builder(api_key).build()
    }

    /// Create a builder for configuring the client.
    ///
    /// Use the builder for custom API URLs, timeouts, or other settings.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    /// use std::time::Duration;
    ///
    /// let client = ClaudeClient::builder("api-key")
    ///     .timeout(Duration::from_secs(60))
    ///     .build();
    /// ```
    pub fn builder(api_key: impl Into<String>) -> ClaudeClientBuilder {
        ClaudeClientBuilder {
            api_key: api_key.into(),
            api_url: DEFAULT_API_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    /// Get the API key (primarily for testing).
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    ///
    /// let client = ClaudeClient::new("my-key");
    /// assert_eq!(client.api_key(), "my-key");
    /// ```
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the API URL.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    ///
    /// let client = ClaudeClient::new("key");
    /// assert!(client.api_url().contains("anthropic.com"));
    /// ```
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Send a message to the Claude API and receive a response.
    ///
    /// This method sends a conversation to Claude and returns the assistant's
    /// response. The request includes all messages in the conversation for
    /// context.
    ///
    /// # Arguments
    ///
    /// * `request` - The message request containing the conversation
    ///
    /// # Errors
    ///
    /// Returns [`FerrisError::Claude`] if:
    /// - The HTTP request fails (network error)
    /// - The API returns a non-2xx status code
    /// - The response cannot be parsed as JSON
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jules_control_plane::llm::claude::ClaudeClient;
    /// use jules_control_plane::llm::types::{CreateMessageRequest, Message};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> jules_control_plane::error::Result<()> {
    /// let client = ClaudeClient::new("api-key");
    ///
    /// let request = CreateMessageRequest::new(vec![
    ///     Message::user("Hello!")
    /// ]);
    ///
    /// let response = client.send_message(request).await?;
    /// println!("Response: {}", response.get_text());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(&self, request: CreateMessageRequest) -> Result<MessageResponse> {
        let response = self
            .http_client
            .post(&self.api_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| FerrisError::Claude(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(FerrisError::Claude(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        let message_response = response
            .json::<MessageResponse>()
            .await
            .map_err(|e| FerrisError::Claude(format!("Failed to parse response: {}", e)))?;

        Ok(message_response)
    }
}

/// Builder for configuring a [`ClaudeClient`].
///
/// # Example
///
/// ```
/// use jules_control_plane::llm::claude::ClaudeClient;
/// use std::time::Duration;
///
/// let client = ClaudeClient::builder("api-key")
///     .api_url("https://proxy.example.com/v1/messages")
///     .timeout(Duration::from_secs(120))
///     .build();
///
/// assert_eq!(client.api_url(), "https://proxy.example.com/v1/messages");
/// ```
pub struct ClaudeClientBuilder {
    api_key: String,
    api_url: String,
    timeout: Duration,
}

impl ClaudeClientBuilder {
    /// Set a custom API URL.
    ///
    /// Useful for testing with mock servers or using a proxy.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    ///
    /// let client = ClaudeClient::builder("key")
    ///     .api_url("http://localhost:8080/v1/messages")
    ///     .build();
    /// ```
    pub fn api_url(mut self, url: impl Into<String>) -> Self {
        self.api_url = url.into();
        self
    }

    /// Set the request timeout.
    ///
    /// Default is 30 seconds. Increase for longer conversations
    /// or slower network connections.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::llm::claude::ClaudeClient;
    /// use std::time::Duration;
    ///
    /// let client = ClaudeClient::builder("key")
    ///     .timeout(Duration::from_secs(60))
    ///     .build();
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the [`ClaudeClient`].
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client cannot be constructed (extremely rare).
    pub fn build(self) -> ClaudeClient {
        let http_client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .expect("Failed to build HTTP client");

        ClaudeClient {
            api_key: self.api_key,
            api_url: self.api_url,
            http_client,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_has_reqwest_client() {
        let client = ClaudeClient::new("test-key");
        let _ = &client;
    }

    #[test]
    fn test_client_builder() {
        let client = ClaudeClient::builder("test-key")
            .api_url("https://custom.api.com/v1/messages")
            .timeout(Duration::from_secs(60))
            .build();

        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.api_url(), "https://custom.api.com/v1/messages");
    }

    #[test]
    fn test_client_default_url() {
        let client = ClaudeClient::new("test-key");
        assert_eq!(client.api_url(), DEFAULT_API_URL);
    }

    #[tokio::test]
    async fn test_send_message_with_mock_server() {
        use crate::llm::types::{CreateMessageRequest, Message};
        use wiremock::matchers::{header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        let mock_response = serde_json::json!({
            "id": "msg_test123",
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": "Hello from mock!"
            }],
            "model": "claude-sonnet-4-20250514",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 10,
                "output_tokens": 5
            }
        });

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .and(header("x-api-key", "test-api-key"))
            .and(header("anthropic-version", "2023-06-01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        let client = ClaudeClient::builder("test-api-key")
            .api_url(format!("{}/v1/messages", mock_server.uri()))
            .build();

        let request = CreateMessageRequest::new(vec![Message::user("Test message")]);
        let response = client
            .send_message(request)
            .await
            .expect("should send message");

        assert_eq!(response.get_text(), "Hello from mock!");
        assert_eq!(response.id, "msg_test123");
    }

    #[tokio::test]
    async fn test_send_message_error_handling() {
        use crate::llm::types::{CreateMessageRequest, Message};
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": {
                    "type": "authentication_error",
                    "message": "Invalid API key"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ClaudeClient::builder("invalid-key")
            .api_url(format!("{}/v1/messages", mock_server.uri()))
            .build();

        let request = CreateMessageRequest::new(vec![Message::user("Test")]);
        let result = client.send_message(request).await;

        assert!(result.is_err());
        if let Err(FerrisError::Claude(msg)) = result {
            assert!(msg.contains("401"));
        } else {
            panic!("Expected Claude error");
        }
    }
}
