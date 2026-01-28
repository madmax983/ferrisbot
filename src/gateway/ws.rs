//! WebSocket handler and protocol for monitoring.
//!
//! This module provides WebSocket support for real-time monitoring
//! and control of the Ferrisbot application.
//!
//! # Protocol
//!
//! The WebSocket uses a JSON protocol with tagged unions. Each message
//! has a `type` field that determines its structure.
//!
//! ## Commands (Client → Server)
//!
//! | Type | Description |
//! |------|-------------|
//! | `ping` | Health check |
//! | `status` | Request bot status |
//!
//! ## Responses (Server → Client)
//!
//! | Type | Fields | Description |
//! |------|--------|-------------|
//! | `pong` | - | Response to ping |
//! | `status` | `bot_connected` | Bot connection status |
//! | `error` | `message` | Error message |
//!
//! # Example Session
//!
//! ```text
//! Client: {"type":"ping"}
//! Server: {"type":"pong"}
//!
//! Client: {"type":"status"}
//! Server: {"type":"status","bot_connected":true}
//!
//! Client: {"type":"invalid"}
//! Server: {"type":"error","message":"Invalid command: unknown variant `invalid`"}
//! ```
//!
//! # Usage
//!
//! Connect using any WebSocket client:
//!
//! ```bash
//! $ wscat -c ws://localhost:18789/ws
//! Connected
//! > {"type":"ping"}
//! < {"type":"pong"}
//! ```

use axum::{
    extract::ws::{Message, WebSocket},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

/// WebSocket command from client.
///
/// Commands are sent by clients to request information or trigger actions.
/// The `type` field determines which command is being sent.
///
/// # Variants
///
/// - `Ping` - Request a pong response (health check)
/// - `Status` - Request current bot status
///
/// # Example
///
/// ```
/// use jules_control_plane::gateway::ws::WsCommand;
///
/// let json = r#"{"type":"ping"}"#;
/// let cmd: WsCommand = serde_json::from_str(json).unwrap();
/// assert!(matches!(cmd, WsCommand::Ping));
/// ```
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WsCommand {
    /// Ping command for health checks.
    #[serde(rename = "ping")]
    Ping,

    /// Status request command.
    #[serde(rename = "status")]
    Status,
}

/// WebSocket response to client.
///
/// Responses are sent by the server in reply to commands.
/// The `type` field determines the response structure.
///
/// # Variants
///
/// - `Pong` - Response to a ping command
/// - `Status` - Bot status information
/// - `Error` - Error message
///
/// # Example
///
/// ```
/// use jules_control_plane::gateway::ws::WsResponse;
///
/// let response = WsResponse::Pong;
/// let json = serde_json::to_string(&response).unwrap();
/// assert!(json.contains(r#""type":"pong""#));
/// ```
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum WsResponse {
    /// Pong response to ping.
    #[serde(rename = "pong")]
    Pong,

    /// Status response with bot information.
    #[serde(rename = "status")]
    Status {
        /// Whether the Discord bot is connected.
        bot_connected: bool,
    },

    /// Error response.
    #[serde(rename = "error")]
    Error {
        /// Error message describing what went wrong.
        message: String,
    },
}

/// Handle WebSocket upgrade request.
///
/// This function is called by Axum when a client requests a WebSocket
/// upgrade on the `/ws` endpoint. It performs the upgrade and hands
/// off to an internal handler for message processing.
///
/// # Arguments
///
/// * `ws` - The WebSocket upgrade extractor from Axum
///
/// # Returns
///
/// An HTTP response that upgrades the connection to WebSocket.
pub async fn ws_handler(ws: axum::extract::ws::WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

/// Handle an active WebSocket connection.
///
/// Processes incoming messages and sends appropriate responses.
/// The connection is maintained until the client disconnects or
/// an error occurs.
async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket client connected");

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    debug!("Received WS message: {}", text);

                    let response = match serde_json::from_str::<WsCommand>(&text) {
                        Ok(WsCommand::Ping) => serde_json::to_string(&WsResponse::Pong).unwrap(),
                        Ok(WsCommand::Status) => {
                            // For MVP, report status as connected (will be dynamic later)
                            serde_json::to_string(&WsResponse::Status {
                                bot_connected: true,
                            })
                            .unwrap()
                        }
                        Err(e) => serde_json::to_string(&WsResponse::Error {
                            message: format!("Invalid command: {}", e),
                        })
                        .unwrap(),
                    };

                    if socket.send(Message::Text(response.into())).await.is_err() {
                        error!("Failed to send WS response");
                        break;
                    }
                }
                Message::Close(_) => {
                    info!("WebSocket client disconnected");
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_command_deserialization() {
        let ping_json = r#"{"type":"ping"}"#;
        let cmd: WsCommand = serde_json::from_str(ping_json).unwrap();
        assert!(matches!(cmd, WsCommand::Ping));

        let status_json = r#"{"type":"status"}"#;
        let cmd: WsCommand = serde_json::from_str(status_json).unwrap();
        assert!(matches!(cmd, WsCommand::Status));
    }

    #[test]
    fn test_ws_response_serialization() {
        let pong = WsResponse::Pong;
        let json = serde_json::to_string(&pong).unwrap();
        assert!(json.contains(r#""type":"pong""#));

        let status = WsResponse::Status {
            bot_connected: true,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""type":"status""#));
        assert!(json.contains(r#""bot_connected":true"#));

        let error = WsResponse::Error {
            message: "test error".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains(r#""type":"error""#));
        assert!(json.contains("test error"));
    }

    #[test]
    fn test_ws_command_invalid() {
        let invalid_json = r#"{"type":"unknown"}"#;
        let result = serde_json::from_str::<WsCommand>(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ws_response_status_false() {
        let status = WsResponse::Status {
            bot_connected: false,
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""bot_connected":false"#));
    }
}
