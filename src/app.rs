//! Application orchestration and lifecycle management.
//!
//! This module provides the main application entry point and configuration.
//! It coordinates the Discord bot and HTTP gateway, managing their lifecycle
//! and handling graceful shutdown.
//!
//! # Architecture
//!
//! The application runs two concurrent tasks:
//!
//! 1. **Discord Bot** - Handles incoming messages and sends replies
//! 2. **HTTP Gateway** - Provides health checks and WebSocket monitoring
//!
//! # Example
//!
//! ```no_run
//! use ferrisbot::app::{App, AppConfig};
//!
//! #[tokio::main]
//! async fn main() -> ferrisbot::error::Result<()> {
//!     let config = AppConfig::from_env()?;
//!     let app = App::new(config);
//!     app.run().await
//! }
//! ```

use crate::discord::bot::DiscordBot;
use crate::error::Result;
use crate::gateway::server::{create_addr, create_router, DEFAULT_PORT};
use crate::llm::claude::ClaudeClient;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

/// Application configuration loaded from environment variables.
///
/// This struct holds all configuration needed to run Ferrisbot,
/// including credentials and service ports.
///
/// # Required Environment Variables
///
/// - `DISCORD_TOKEN` - Discord bot token from the Developer Portal
/// - `ANTHROPIC_API_KEY` - API key for Claude
///
/// # Optional Environment Variables
///
/// - `GATEWAY_PORT` - Port for the HTTP gateway (default: 18789)
///
/// # Example
///
/// ```no_run
/// use ferrisbot::app::AppConfig;
///
/// // Load from environment
/// let config = AppConfig::from_env().expect("Missing required env vars");
///
/// println!("Gateway will run on port {}", config.gateway_port);
/// ```
///
/// Or create manually for testing:
///
/// ```
/// use ferrisbot::app::AppConfig;
///
/// let config = AppConfig {
///     discord_token: "test-token".to_string(),
///     claude_api_key: "test-key".to_string(),
///     gateway_port: 8080,
/// };
///
/// assert_eq!(config.gateway_port, 8080);
/// ```
pub struct AppConfig {
    /// Discord bot token for authentication.
    pub discord_token: String,

    /// Anthropic API key for Claude.
    pub claude_api_key: String,

    /// Port for the HTTP gateway server.
    pub gateway_port: u16,
}

impl AppConfig {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`FerrisError::Config`](crate::error::FerrisError::Config) if:
    /// - `DISCORD_TOKEN` is not set
    /// - `ANTHROPIC_API_KEY` is not set
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ferrisbot::app::AppConfig;
    ///
    /// let config = AppConfig::from_env()?;
    /// # Ok::<(), ferrisbot::error::FerrisError>(())
    /// ```
    pub fn from_env() -> Result<Self> {
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| crate::error::FerrisError::Config("DISCORD_TOKEN not set".to_string()))?;

        let claude_api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| {
            crate::error::FerrisError::Config("ANTHROPIC_API_KEY not set".to_string())
        })?;

        let gateway_port = env::var("GATEWAY_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        Ok(Self {
            discord_token,
            claude_api_key,
            gateway_port,
        })
    }
}

/// Main application that orchestrates all services.
///
/// The `App` struct manages the lifecycle of the Discord bot and HTTP gateway,
/// running them concurrently and handling graceful shutdown on SIGINT (Ctrl+C).
///
/// # Example
///
/// ```no_run
/// use ferrisbot::app::{App, AppConfig};
///
/// #[tokio::main]
/// async fn main() -> ferrisbot::error::Result<()> {
///     let config = AppConfig {
///         discord_token: std::env::var("DISCORD_TOKEN").unwrap(),
///         claude_api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
///         gateway_port: 18789,
///     };
///
///     let app = App::new(config);
///     app.run().await
/// }
/// ```
pub struct App {
    config: AppConfig,
    discord_bot: DiscordBot,
    claude_client: Arc<ClaudeClient>,
}

impl App {
    /// Create a new application with the given configuration.
    ///
    /// This initializes the Discord bot and Claude client but does not
    /// start any services. Call [`run`](App::run) to start the application.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisbot::app::{App, AppConfig};
    ///
    /// let config = AppConfig {
    ///     discord_token: "test-token".to_string(),
    ///     claude_api_key: "test-key".to_string(),
    ///     gateway_port: 8080,
    /// };
    ///
    /// let app = App::new(config);
    /// ```
    pub fn new(config: AppConfig) -> Self {
        let discord_bot = DiscordBot::new(&config.discord_token);
        let claude_client = Arc::new(ClaudeClient::new(&config.claude_api_key));

        Self {
            config,
            discord_bot,
            claude_client,
        }
    }

    /// Run the full application (gateway + Discord bot).
    ///
    /// This method starts both the Discord bot and HTTP gateway as concurrent
    /// tasks. It handles graceful shutdown when:
    ///
    /// - Either task completes (normally or with error)
    /// - A SIGINT signal is received (Ctrl+C)
    ///
    /// # Note
    ///
    /// This method takes ownership of `self` because the Discord bot
    /// must be moved into a spawned task.
    ///
    /// # Errors
    ///
    /// Returns an error if the Discord client fails to initialize.
    pub async fn run(self) -> Result<()> {
        // Create shutdown channel for Discord bot
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

        // Use the stored bot and client
        let bot = self.discord_bot;
        let claude_client = self.claude_client;

        // Start Discord bot in background
        let bot_handle = tokio::spawn(async move {
            info!("Starting Discord bot...");
            if let Err(e) = bot.start(claude_client, shutdown_rx).await {
                tracing::error!("Discord bot error: {}", e);
            }
        });

        // Start gateway server
        let port = self.config.gateway_port;
        let gateway_handle = tokio::spawn({
            async move {
                let app = create_router();
                let addr = create_addr(port);

                info!("Starting gateway on {}", addr);

                let listener = match tokio::net::TcpListener::bind(addr).await {
                    Ok(l) => l,
                    Err(e) => {
                        tracing::error!("Failed to bind gateway: {}", e);
                        return;
                    }
                };

                if let Err(e) = axum::serve(listener, app).await {
                    tracing::error!("Gateway error: {}", e);
                }
            }
        });

        // Wait for either task to complete (or Ctrl+C)
        tokio::select! {
            _ = bot_handle => {
                info!("Discord bot task completed");
            }
            _ = gateway_handle => {
                info!("Gateway task completed");
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Received shutdown signal");
                // Signal bot to shutdown
                let _ = shutdown_tx.send(()).await;
            }
        }

        Ok(())
    }

    /// Start the gateway server only (for testing).
    ///
    /// This runs only the HTTP gateway without the Discord bot,
    /// useful for testing the gateway in isolation.
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to bind or encounters an error.
    pub async fn run_gateway(self) -> Result<()> {
        let app = create_router();
        let addr = create_addr(self.config.gateway_port);

        info!("Starting gateway on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| crate::error::FerrisError::Config(format!("Failed to bind: {}", e)))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| crate::error::FerrisError::Config(format!("Server error: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let config = AppConfig {
            discord_token: "test-token".to_string(),
            claude_api_key: "test-key".to_string(),
            gateway_port: 8080,
        };

        let app = App::new(config);
        assert_eq!(app.config.gateway_port, 8080);
    }

    #[test]
    fn test_app_config_from_env_missing() {
        // Clear env vars
        env::remove_var("DISCORD_TOKEN");
        env::remove_var("ANTHROPIC_API_KEY");

        let result = AppConfig::from_env();
        assert!(result.is_err());
    }
}
