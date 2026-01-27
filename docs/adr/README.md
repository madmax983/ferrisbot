# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for Ferrisbot.

## What is an ADR?

An ADR is a document that captures an important architectural decision made along with its context and consequences.

## Format

Each ADR follows this structure:

```markdown
# ADR-NNN: Title

## Status
[Proposed | Accepted | Deprecated | Superseded]

## Context
What is the issue that we're seeing that is motivating this decision or change?

## Decision
What is the change that we're proposing and/or doing?

## Consequences
What becomes easier or more difficult to do because of this change?
```

## Index

- [ADR-001: Use Rust for Implementation](./001-use-rust.md)
- [ADR-002: Strict Test-Driven Development](./002-strict-tdd.md)
- [ADR-003: Serenity for Discord Integration](./003-serenity-discord.md)
- [ADR-004: Axum for HTTP Gateway](./004-axum-gateway.md)
- [ADR-005: Single Error Type with thiserror](./005-single-error-type.md)
- [ADR-006: Tokio Async Runtime](./006-tokio-runtime.md)
- [ADR-007: Modular Architecture](./007-modular-architecture.md)
- [ADR-008: Environment-Based Configuration](./008-env-config.md)
- [ADR-009: WebSocket JSON Protocol](./009-websocket-json.md)
- [ADR-010: MVP-First Approach](./010-mvp-first.md)

## Creating New ADRs

1. Copy the template from `000-template.md`
2. Number sequentially (e.g., ADR-011)
3. Fill in all sections
4. Update this index
5. Submit for review
