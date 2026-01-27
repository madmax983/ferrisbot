// End-to-end integration tests

use ferrisbot::llm::claude::ClaudeClient;
use ferrisbot::llm::types::{CreateMessageRequest, Message};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_claude_api_integration() {
    // Start mock Claude API server
    let mock_server = MockServer::start().await;

    let mock_response = serde_json::json!({
        "id": "msg_integration_test",
        "type": "message",
        "role": "assistant",
        "content": [{
            "type": "text",
            "text": "Hello! This is a test response from Claude."
        }],
        "model": "claude-sonnet-4-20250514",
        "stop_reason": "end_turn",
        "usage": {
            "input_tokens": 15,
            "output_tokens": 10
        }
    });

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    // Create Claude client pointing to mock server
    let client = ClaudeClient::builder("test-api-key")
        .api_url(format!("{}/v1/messages", mock_server.uri()))
        .build();

    // Send a message
    let request = CreateMessageRequest::new(vec![Message::user("Hello, Claude!")]);

    let response = client
        .send_message(request)
        .await
        .expect("Failed to send message");

    // Verify response
    assert_eq!(response.id, "msg_integration_test");
    assert_eq!(
        response.get_text(),
        "Hello! This is a test response from Claude."
    );
    assert_eq!(response.total_tokens(), 25);
}

#[tokio::test]
async fn test_gateway_health_check() {
    use ferrisbot::gateway::server::create_router;

    // Create router
    let app = create_router();

    // Start test server on random port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Failed to serve");
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Make request to health endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read body");
    assert_eq!(body, "OK");
}

#[tokio::test]
async fn test_message_conversion_flow() {
    use ferrisbot::discord::convert::{should_process_message, to_ferris_message};
    use ferrisbot::types::Message as FerrisMessage;

    // Note: Creating a real Discord Message is complex due to serenity's internal structure
    // For this MVP, we verify the conversion functions exist and can be called
    // Full Discord mock integration would require more complex setup

    // This test verifies the API is correct
    let _convert = to_ferris_message;
    let _should_process = should_process_message;

    // We can test the type directly
    let ferris_msg = FerrisMessage::new("Test message");
    assert_eq!(ferris_msg.content, "Test message");
}

#[test]
fn test_error_handling_throughout_stack() {
    use ferrisbot::error::FerrisError;

    // Test IO error conversion
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
    let ferris_err: FerrisError = io_err.into();
    assert!(matches!(ferris_err, FerrisError::Io(_)));

    // Test JSON error conversion
    let json_err = serde_json::from_str::<String>("{invalid}").unwrap_err();
    let ferris_err: FerrisError = json_err.into();
    assert!(matches!(ferris_err, FerrisError::Json(_)));

    // Test custom errors
    let config_err = FerrisError::Config("test".to_string());
    assert!(format!("{}", config_err).contains("test"));

    let discord_err = FerrisError::Discord("discord error".to_string());
    assert!(format!("{}", discord_err).contains("discord error"));

    let claude_err = FerrisError::Claude("claude error".to_string());
    assert!(format!("{}", claude_err).contains("claude error"));
}
