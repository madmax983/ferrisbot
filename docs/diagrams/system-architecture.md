# System Architecture

Overall architecture of Ferrisbot v0.1.0 MVP.

```mermaid
graph TB
    subgraph "External Services"
        Discord[Discord API<br/>WebSocket Gateway]
        Claude[Claude API<br/>HTTPS REST]
    end

    subgraph "Ferrisbot Application"
        subgraph "Main Process"
            Main[main.rs<br/>Application Entry]
            App[app.rs<br/>Orchestration]
        end

        subgraph "Discord Layer"
            Bot[DiscordBot<br/>Serenity Client]
            Handler[BotHandler<br/>Event Handler]
            Convert[Type Conversion<br/>Discord ↔ Internal]
        end

        subgraph "Gateway Layer"
            Server[HTTP Server<br/>Axum Router]
            Routes[Routes<br/>/health, /ws]
            WS[WebSocket Handler<br/>JSON Protocol]
        end

        subgraph "LLM Layer"
            Client[ClaudeClient<br/>HTTP Client]
            Types[Request/Response<br/>Types]
        end

        subgraph "Foundation"
            CoreTypes[Core Types<br/>Message]
            Errors[Error Handling<br/>FerrisError]
        end
    end

    subgraph "Monitoring"
        User[Admin User<br/>WebSocket Client]
    end

    Discord <-->|Gateway Events| Bot
    Bot --> Handler
    Handler --> Convert
    Convert --> Client
    Client <-->|HTTPS| Claude

    Main --> App
    App -->|Spawn Task| Bot
    App -->|Spawn Task| Server

    Server --> Routes
    Routes --> WS
    User <-->|WebSocket| WS

    Handler --> CoreTypes
    Client --> Types
    Types --> CoreTypes
    Bot --> Errors
    Client --> Errors
    Server --> Errors

    classDef external fill:#f9f,stroke:#333,stroke-width:2px
    classDef core fill:#bbf,stroke:#333,stroke-width:2px
    classDef layer fill:#bfb,stroke:#333,stroke-width:2px

    class Discord,Claude external
    class CoreTypes,Errors core
    class Bot,Handler,Convert,Server,Routes,WS,Client,Types layer
```

## Description

**External Services:**
- **Discord API**: WebSocket gateway for real-time events
- **Claude API**: HTTPS REST API for AI completions

**Main Process:**
- **main.rs**: Application startup and configuration loading
- **app.rs**: Orchestrates Discord bot and gateway server as concurrent tasks

**Discord Layer** (Serenity):
- **DiscordBot**: Manages connection lifecycle and configuration
- **BotHandler**: Implements EventHandler trait for message events
- **Convert**: Transforms Discord types to internal Message types

**Gateway Layer** (Axum):
- **HTTP Server**: Serves health checks and WebSocket upgrades
- **Routes**: Defines /health and /ws endpoints
- **WebSocket Handler**: JSON protocol for monitoring and control

**LLM Layer** (Reqwest):
- **ClaudeClient**: HTTP client for Claude API
- **Request/Response Types**: Strongly-typed API messages

**Foundation**:
- **Core Types**: Shared domain types (Message)
- **Error Handling**: Unified error type (FerrisError)

## Key Design Decisions

1. **Concurrent Tasks**: Bot and Gateway run as separate Tokio tasks
2. **Shared Runtime**: All components use Tokio for async operations
3. **Type Safety**: Strong typing throughout with compile-time checks
4. **Layered Architecture**: Clear separation of concerns
5. **Foundation Types**: Shared error and message types reduce coupling

## Related

- ADR-007: Modular Architecture
- ADR-006: Tokio Runtime
- [Component Diagram](./components.md) for detailed module view
