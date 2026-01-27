# ADR-007: Modular Architecture

## Status

Accepted

Date: 2026-01-26

## Context

We need to organize code for:
- **Separation of concerns**: Each module has one responsibility
- **Testability**: Easy to test in isolation
- **Maintainability**: Easy to find and modify code
- **Extensibility**: Easy to add new features
- **Reusability**: Library + binary structure

Alternatives considered:
- **Monolithic**: All code in main.rs/lib.rs (doesn't scale)
- **Feature-based**: Modules by feature (chat, admin, etc.)
- **Layer-based**: Modules by technical layer (our choice)
- **Domain-driven**: Complex bounded contexts (overkill for MVP)

## Decision

We will use a **layer-based modular architecture** with clear boundaries.

Structure:
```
src/
├── lib.rs              # Library public API
├── main.rs             # Binary entry point
├── app.rs              # Application orchestration
├── error.rs            # Shared error types
├── types/              # Domain types
├── llm/                # LLM integration layer
│   ├── claude.rs
│   └── types.rs
├── discord/            # Discord integration layer
│   ├── bot.rs
│   ├── handler.rs
│   └── convert.rs
└── gateway/            # HTTP/WebSocket layer
    ├── server.rs
    ├── routes.rs
    └── ws.rs
```

Principles:
- **One responsibility** per module
- **Clear interfaces** between modules
- **No circular dependencies**
- **Types flow** one direction
- **Shared code** in lib.rs

Dependency flow:
```
main.rs
  ↓
app.rs (orchestration)
  ↓
discord + gateway (parallel)
  ↓
llm (shared)
  ↓
types + error (foundation)
```

## Consequences

### Positive

- **Testability**: Mock dependencies easily
- **Parallel development**: Teams can work on different modules
- **Clear ownership**: Each module has defined purpose
- **Easy navigation**: Find code quickly
- **Refactoring**: Change internals without affecting others
- **Reusability**: Can use as library

### Negative

- **More files**: Boilerplate for module structure
- **Indirection**: Following code paths across modules
- **Integration complexity**: Wiring modules together

### Neutral

- **File count**: More files but better organization
- **Import statements**: More `use` statements needed

## Notes

This modular approach aligned perfectly with TDD:
- Each module tested independently
- Clear interfaces made mocking straightforward
- Integration tests verify module interactions

The library/binary split (`lib.rs` + `main.rs`) provides:
- Reusable library for other projects
- Testable business logic
- Thin binary for orchestration

Example of clear boundaries:
```rust
// discord/convert.rs knows about Discord types
// llm/types.rs knows about Claude types
// Neither knows about the other
// app.rs coordinates between them
```

This structure scales well to v0.2.0+ features:
- Add `database/` for persistence
- Add `agents/` for agent framework
- Add `tools/` for function calling

## References

- [Rust Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
