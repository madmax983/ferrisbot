# Ferrisbot Documentation Index

Complete guide to Ferrisbot's documentation.

## Getting Started

**New to Ferrisbot?** Start here:

1. **[README](../README.md)** - Project overview and quick facts
2. **[QUICKSTART](../QUICKSTART.md)** - Setup guide and first run
3. **[CLAUDE.md](../CLAUDE.md)** - Development guide (for AI assistants and developers)

## Planning & Vision

**Understanding the project direction:**

- **[ROADMAP](../ROADMAP.md)** - Feature development plan (v0.1.0 → v1.0.0+)
- **[VERIFICATION_CHECKLIST](../VERIFICATION_CHECKLIST.md)** - Pre-deployment checks

## Architecture Documentation

### Architecture Decision Records (ADRs)

**Why we built it this way:**

- [ADR Index](./adr/README.md) - All architecture decisions
- [ADR-001: Use Rust](./adr/001-use-rust.md) - Language choice
- [ADR-002: Strict TDD](./adr/002-strict-tdd.md) - Development methodology
- [ADR-003: Serenity for Discord](./adr/003-serenity-discord.md) - Discord library
- [ADR-004: Axum for Gateway](./adr/004-axum-gateway.md) - HTTP framework
- [ADR-005: Single Error Type](./adr/005-single-error-type.md) - Error handling strategy
- [ADR-006: Tokio Runtime](./adr/006-tokio-runtime.md) - Async runtime
- [ADR-007: Modular Architecture](./adr/007-modular-architecture.md) - Code organization
- [ADR-008: Environment Config](./adr/008-env-config.md) - Configuration approach
- [ADR-009: WebSocket JSON Protocol](./adr/009-websocket-json.md) - Gateway protocol
- [ADR-010: MVP-First Approach](./adr/010-mvp-first.md) - Development strategy

### Architecture Diagrams

**Visual system documentation:**

- [Diagram Index](./diagrams/README.md) - All diagrams
- [System Architecture](./diagrams/system-architecture.md) - Overall system design
- [Message Flow](./diagrams/message-flow.md) - End-to-end sequence
- [Component Diagram](./diagrams/components.md) - Module relationships
- [Deployment](./diagrams/deployment.md) - Runtime deployment
- [State Machine](./diagrams/state-machine.md) - Message processing states

## Implementation Details

**Deep dives into the codebase:**

- **[IMPLEMENTATION_SUMMARY](../IMPLEMENTATION_SUMMARY.md)** - Complete phase-by-phase breakdown
  - Phase 1: Foundation (types, errors)
  - Phase 2: Claude API Client
  - Phase 3: Discord Integration
  - Phase 4: Gateway (HTTP/WebSocket)
  - Phase 5: Integration & Orchestration

## For Contributors

**Adding to the project:**

1. Read [CLAUDE.md](../CLAUDE.md) - Understand TDD approach
2. Review [ROADMAP.md](../ROADMAP.md) - See what's planned
3. Check [ADRs](./adr/README.md) - Understand past decisions
4. Follow the RED-GREEN-REFACTOR cycle
5. Update documentation as you go

## For Operators

**Running in production:**

- [QUICKSTART.md](../QUICKSTART.md) - Setup and configuration
- [Deployment Diagram](./diagrams/deployment.md) - Deployment options
- [VERIFICATION_CHECKLIST](../VERIFICATION_CHECKLIST.md) - Pre-deployment checks

## Documentation Standards

### When to Create ADRs

Create an ADR when making decisions about:
- Technology choices (libraries, frameworks)
- Architectural patterns
- Development practices
- Protocol designs
- Major refactorings

Use the [template](./adr/000-template.md).

### When to Create Diagrams

Create diagrams for:
- New major features
- System redesigns
- Complex interactions
- Onboarding new developers

Use Mermaid format for version control.

### When to Update ROADMAP

Update the roadmap when:
- Completing a version
- Changing priorities
- Adding new features
- Discovering technical constraints

## Document Ownership

| Document | Owner | Update Frequency |
|----------|-------|------------------|
| README.md | Maintainers | Every release |
| ROADMAP.md | Product | Monthly |
| ADRs | Architects | Per decision |
| Diagrams | Architects | Per architecture change |
| CLAUDE.md | Tech Lead | As methodology evolves |
| QUICKSTART.md | DevOps | Per deployment change |

## Finding Information

**Looking for...**

- **How to set up?** → [QUICKSTART.md](../QUICKSTART.md)
- **Why is it built this way?** → [ADRs](./adr/README.md)
- **How does it work?** → [Diagrams](./diagrams/README.md)
- **What's next?** → [ROADMAP.md](../ROADMAP.md)
- **How to develop?** → [CLAUDE.md](../CLAUDE.md)
- **What was built?** → [IMPLEMENTATION_SUMMARY.md](../IMPLEMENTATION_SUMMARY.md)

## Contributing to Documentation

1. Follow existing formats
2. Use Mermaid for diagrams
3. Create ADR for architectural decisions
4. Update index files
5. Keep examples up-to-date
6. Test all code examples

## Version

This documentation is for **Ferrisbot v0.1.0 MVP**.

Last updated: 2026-01-26
