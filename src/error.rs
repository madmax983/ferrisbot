//! Error handling for Ferrisbot.
//!
//! This module provides a unified error type [`FerrisError`] that encompasses
//! all possible errors in the application. It uses the [`thiserror`] crate
//! for ergonomic error handling with automatic `Display` and `Error` implementations.
//!
//! # Example
//!
//! ```
//! use jules_control_plane::error::{FerrisError, Result};
//!
//! fn load_config(key: &str) -> Result<String> {
//!     if key.is_empty() {
//!         return Err(FerrisError::Config("Key cannot be empty".to_string()));
//!     }
//!     Ok("value".to_string())
//! }
//!
//! // Using the ? operator for propagation
//! fn process() -> Result<()> {
//!     let _value = load_config("my_key")?;
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// Main error type for Ferrisbot.
///
/// This enum represents all possible errors that can occur in the application.
/// It implements `std::error::Error` and `Display` via the `thiserror` derive macro.
///
/// # Variants
///
/// - [`Config`](FerrisError::Config) - Configuration errors (missing env vars, invalid values)
/// - [`Io`](FerrisError::Io) - I/O errors (file operations, network issues)
/// - [`Json`](FerrisError::Json) - JSON serialization/deserialization errors
/// - [`Discord`](FerrisError::Discord) - Discord API errors
/// - [`Claude`](FerrisError::Claude) - Claude API errors
/// - [`WebSocket`](FerrisError::WebSocket) - WebSocket connection errors
///
/// # Example
///
/// ```
/// use jules_control_plane::error::FerrisError;
///
/// // Create a configuration error
/// let err = FerrisError::Config("DISCORD_TOKEN not set".to_string());
/// assert!(err.to_string().contains("DISCORD_TOKEN"));
///
/// // Errors can be matched for specific handling
/// match err {
///     FerrisError::Config(msg) => println!("Config error: {}", msg),
///     FerrisError::Claude(msg) => println!("Claude error: {}", msg),
///     _ => println!("Other error"),
/// }
/// ```
#[derive(Debug, Error)]
pub enum FerrisError {
    /// Configuration error (missing or invalid configuration).
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::error::FerrisError;
    ///
    /// let err = FerrisError::Config("API key not provided".to_string());
    /// assert_eq!(err.to_string(), "Configuration error: API key not provided");
    /// ```
    #[error("Configuration error: {0}")]
    Config(String),

    /// I/O error from file or network operations.
    ///
    /// This variant automatically converts from [`std::io::Error`].
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error.
    ///
    /// This variant automatically converts from [`serde_json::Error`].
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Discord API or connection error.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::error::FerrisError;
    ///
    /// let err = FerrisError::Discord("Connection timeout".to_string());
    /// assert!(err.to_string().contains("Discord"));
    /// ```
    #[error("Discord error: {0}")]
    Discord(String),

    /// Claude API error.
    ///
    /// # Example
    ///
    /// ```
    /// use jules_control_plane::error::FerrisError;
    ///
    /// let err = FerrisError::Claude("Rate limit exceeded".to_string());
    /// assert!(err.to_string().contains("Claude"));
    /// ```
    #[error("Claude API error: {0}")]
    Claude(String),

    /// WebSocket connection or protocol error.
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

/// A specialized `Result` type for Ferrisbot operations.
///
/// This type alias simplifies function signatures throughout the codebase
/// by providing a default error type of [`FerrisError`].
///
/// # Example
///
/// ```
/// use jules_control_plane::error::Result;
///
/// fn do_something() -> Result<String> {
///     Ok("success".to_string())
/// }
///
/// assert!(do_something().is_ok());
/// ```
pub type Result<T> = std::result::Result<T, FerrisError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ferris_error_exists() {
        let _err: FerrisError;
    }

    #[test]
    fn test_error_conversion_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let ferris_err: FerrisError = io_err.into();
        assert!(matches!(ferris_err, FerrisError::Io(_)));
    }

    #[test]
    fn test_error_conversion_from_serde() {
        let json_str = "{invalid json}";
        let serde_err = serde_json::from_str::<String>(json_str).unwrap_err();
        let ferris_err: FerrisError = serde_err.into();
        assert!(matches!(ferris_err, FerrisError::Json(_)));
    }

    #[test]
    fn test_error_display() {
        let err = FerrisError::Config("test config error".to_string());
        let display_str = format!("{}", err);
        assert!(display_str.contains("test config error"));
    }
}
