use jules_control_plane::llm::types::{CreateMessageRequest, Message};

#[test]
fn test_jules_persona_request_creation() {
    let content = "Hello";
    let request = CreateMessageRequest::new(vec![Message::user(content)])
        .with_system("You are Jules, an extremely skilled software engineer. You are operating as a control plane via Discord.");

    assert_eq!(request.system, Some("You are Jules, an extremely skilled software engineer. You are operating as a control plane via Discord.".to_string()));
}
