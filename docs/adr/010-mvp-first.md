# ADR-010: MVP-First Approach

## Status

Accepted

Date: 2026-01-26

## Context

We need to decide on the initial scope and delivery strategy. Options:

- **Full-featured v1.0**: Build all planned features before release (6+ months)
- **MVP then iterate**: Build minimal working version, then add features (our choice)
- **Spike then rebuild**: Quick prototype, then production version (wasteful)

Requirements for success:
- Prove the architecture works
- Ship something usable quickly
- Enable future extensibility
- Validate technology choices
- Build foundation for growth

## Decision

We will build an **MVP (Minimum Viable Product)** first, labeled v0.1.0, with only essential features.

**MVP Scope (v0.1.0):**
- ✅ Discord message handling
- ✅ Claude API integration
- ✅ Basic chat flow (user → bot → Claude → reply)
- ✅ HTTP gateway with health check
- ✅ WebSocket for monitoring
- ✅ Error handling
- ✅ Comprehensive tests
- ✅ Documentation

**Explicitly NOT in MVP:**
- ❌ Conversation history/context
- ❌ Streaming responses
- ❌ Tool use / function calling
- ❌ Agent framework
- ❌ Database persistence
- ❌ Advanced admin features
- ❌ Channel permissions
- ❌ Rate limiting (per-user)

**Post-MVP Roadmap:**
- v0.2.0: Conversation context & history
- v0.3.0: Streaming responses
- v0.4.0: Tool use & function calling
- v0.5.0: Agent framework
- v0.6.0+: Advanced features

## Consequences

### Positive

- **Fast delivery**: MVP completed in single session
- **Architecture validation**: Proved the design works
- **Early feedback**: Can get user input before building more
- **Reduced risk**: Smaller initial investment
- **Clear foundation**: Solid base for future features
- **Momentum**: Working product motivates next steps
- **Learning**: Understand requirements better

### Negative

- **Limited features**: Users may want more immediately
- **Refactoring risk**: May need to change architecture later
- **Marketing**: Harder to promote incomplete product

### Neutral

- **Technical debt**: Accepting some shortcuts for speed (minimal in our case)
- **Version churn**: More frequent releases during iteration

## Notes

The MVP-first approach proved highly successful:

**What we learned:**
- The layered architecture works well
- TDD catches issues early
- Serenity + Tokio + Axum integrate smoothly
- Type system prevents common bugs

**What's ready for extension:**
- Conversation store (v0.2.0)
- Streaming parser (v0.3.0)
- Tool framework (v0.4.0)
- Agent system (v0.5.0)

**Key insight:**
Building the foundation correctly is more valuable than adding features quickly. The MVP has:
- 48 tests (all passing)
- Zero production .unwrap() calls
- Comprehensive error handling
- Clean module boundaries

This foundation makes future features EASIER to add, not harder.

## References

- [The Lean Startup - Eric Ries](https://theleanstartup.com/)
- [MVP Definition - Wikipedia](https://en.wikipedia.org/wiki/Minimum_viable_product)
- Related: ROADMAP.md - Full feature plan
- Related: ADR-002 (TDD) - Quality from the start
