# Component Diagram

Detailed view of module relationships and dependencies.

```mermaid
graph LR
    subgraph "Binary"
        Main[main.rs]
    end

    subgraph "Application Layer"
        App[app.rs<br/>AppConfig, App]
    end

    subgraph "Discord Module"
        DiscordMod[discord/mod.rs]
        Bot[discord/bot.rs<br/>DiscordBot, Builder]
        Handler[discord/handler.rs<br/>BotHandler]
        Convert[discord/convert.rs<br/>Converters, Filters]
    end

    subgraph "Gateway Module"
        GatewayMod[gateway/mod.rs]
        Server[gateway/server.rs<br/>Router, Config]
        Routes[gateway/routes.rs<br/>health()]
        WS[gateway/ws.rs<br/>WebSocket, Commands]
    end

    subgraph "LLM Module"
        LLMMod[llm/mod.rs]
        Claude[llm/claude.rs<br/>ClaudeClient, Builder]
        LLMTypes[llm/types.rs<br/>Request, Response]
    end

    subgraph "Foundation"
        Lib[lib.rs<br/>Public API]
        Types[types/mod.rs<br/>Message]
        Error[error.rs<br/>FerrisError, Result]
    end

    Main -->|uses| App
    App -->|spawns| Bot
    App -->|spawns| Server

    DiscordMod --> Bot
    DiscordMod --> Handler
    DiscordMod --> Convert

    Handler -->|uses| Bot
    Handler -->|uses| Convert
    Handler -->|uses| Claude

    GatewayMod --> Server
    GatewayMod --> Routes
    GatewayMod --> WS

    Server --> Routes
    Server --> WS

    LLMMod --> Claude
    LLMMod --> LLMTypes

    Claude -->|uses| LLMTypes

    Lib -->|exports| DiscordMod
    Lib -->|exports| GatewayMod
    Lib -->|exports| LLMMod
    Lib -->|exports| Types
    Lib -->|exports| Error
    Lib -->|exports| App

    Bot -.->|depends on| Error
    Handler -.->|depends on| Error
    Handler -.->|depends on| Types
    Claude -.->|depends on| Error
    Claude -.->|depends on| Types
    Server -.->|depends on| Error
    Convert -.->|depends on| Types

    classDef binary fill:#faa,stroke:#333,stroke-width:2px
    classDef app fill:#afa,stroke:#333,stroke-width:2px
    classDef module fill:#aaf,stroke:#333,stroke-width:2px
    classDef foundation fill:#ffa,stroke:#333,stroke-width:2px

    class Main binary
    class App app
    class DiscordMod,Bot,Handler,Convert,GatewayMod,Server,Routes,WS,LLMMod,Claude,LLMTypes module
    class Lib,Types,Error foundation
```

## Module Responsibilities

### Binary
- **main.rs**: Entry point, loads config, starts app

### Application Layer
- **app.rs**: Orchestrates lifecycle, manages tasks, handles shutdown

### Discord Module
- **mod.rs**: Public API, exports types
- **bot.rs**: Discord client wrapper, connection management
- **handler.rs**: Event handler, message processing
- **convert.rs**: Type conversions, message filtering

### Gateway Module
- **mod.rs**: Public API, exports server
- **server.rs**: Axum router, socket binding
- **routes.rs**: HTTP route handlers
- **ws.rs**: WebSocket upgrade and protocol

### LLM Module
- **mod.rs**: Public API, exports client
- **claude.rs**: HTTP client, request/response handling
- **types.rs**: API types, serialization

### Foundation
- **lib.rs**: Library root, re-exports public API
- **types/mod.rs**: Core domain types
- **error.rs**: Error types, conversions

## Dependency Rules

1. **No circular dependencies**: Enforced by Rust compiler
2. **Foundation is leaf**: types and error have no dependencies
3. **Modules don't depend on each other**: Only through foundation
4. **App coordinates**: Only app.rs connects modules
5. **Lib.rs is gateway**: Only public interface to library

## Testing Strategy

Each module tested independently:
- **Unit tests**: In same file as implementation
- **Integration tests**: In tests/ directory
- **Mocks**: wiremock for HTTP, manual for Discord

Dependencies mocked using traits (future) or Arc for shared state.

## Related

- ADR-007: Modular Architecture
- ADR-001: Use Rust (module system)
- [System Architecture](./system-architecture.md) for runtime view
