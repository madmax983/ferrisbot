# ADR-009: WebSocket JSON Protocol

## Status

Accepted

Date: 2026-01-26

## Context

The gateway needs a WebSocket protocol for real-time monitoring and control. Requirements:

- **Human-readable**: Easy to debug and test
- **Structured**: Type-safe messages
- **Extensible**: Easy to add new commands
- **Standard**: Use existing protocols where possible

Alternatives considered:
- **JSON**: Human-readable, widely supported (our choice)
- **MessagePack**: Binary, efficient, harder to debug
- **Protocol Buffers**: Strongly-typed, requires codegen
- **Custom binary**: Maximum efficiency, lots of work

## Decision

We will use **JSON with a tagged union protocol** for WebSocket messages.

Message format:
```json
{
  "type": "command_name",
  "field1": "value1",
  "field2": "value2"
}
```

Implementation:
```rust
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WsCommand {
    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "status")]
    Status,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum WsResponse {
    #[serde(rename = "pong")]
    Pong,

    #[serde(rename = "status")]
    Status { bot_connected: bool },

    #[serde(rename = "error")]
    Error { message: String },
}
```

Commands (v0.1.0):
- `ping` → `pong` - Health check
- `status` → `status` - Bot status

Future commands (v0.2.0+):
- `subscribe` - Subscribe to events
- `start_bot` / `stop_bot` - Control bot
- `clear_history` - Clear conversation

## Consequences

### Positive

- **Debuggable**: Use browser DevTools or wscat
- **Language-agnostic**: Any language can connect
- **Type-safe**: Serde validates structure
- **Extensible**: Add new types to enum
- **Standard**: JSON is universal
- **Testing**: Easy to write test cases

### Negative

- **Verbose**: More bytes than binary formats
- **Parsing cost**: JSON parsing overhead
- **No versioning**: Must handle gracefully
- **No streaming**: Each message is complete

### Neutral

- **Performance**: Good enough for monitoring (not high-throughput)
- **Error handling**: Deserialize errors caught cleanly

## Notes

The tagged union pattern (`#[serde(tag = "type")]`) provides:
- Compile-time exhaustive matching
- Runtime type discrimination
- Clear error messages for invalid types

Example usage:
```bash
$ wscat -c ws://localhost:18789/ws
> {"type":"ping"}
< {"type":"pong"}

> {"type":"status"}
< {"type":"status","bot_connected":true}

> {"type":"invalid"}
< {"type":"error","message":"Invalid command: unknown variant `invalid`"}
```

For high-throughput scenarios (v0.3.0+ streaming), we may add binary encoding alongside JSON for efficiency.

## References

- [Serde Tagged Enums](https://serde.rs/enum-representations.html#adjacently-tagged)
- [WebSocket Protocol RFC 6455](https://tools.ietf.org/html/rfc6455)
- Related: ADR-004 (Axum) - WebSocket support
