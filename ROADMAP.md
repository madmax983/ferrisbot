# Ferrisbot Development Roadmap

## Vision

Build a powerful, extensible Discord bot framework powered by Claude AI, with support for agents, tools, streaming, and rich conversation management.

---

## v0.1.0 - MVP Foundation ✅ **COMPLETE**

**Goal:** Prove the architecture works with minimal features.

### Completed Features
- ✅ Core types and error handling
- ✅ Claude API client with mocked tests
- ✅ Discord bot integration (Serenity)
- ✅ Message filtering (bots, empty messages)
- ✅ HTTP gateway with health check
- ✅ WebSocket with JSON protocol
- ✅ Full message flow: Discord → Claude → Reply
- ✅ Concurrent bot + gateway tasks
- ✅ Graceful shutdown
- ✅ 48 tests (44 unit + 4 integration)
- ✅ Comprehensive documentation

### What Works
- User sends message in Discord
- Bot receives and filters message
- Message sent to Claude API
- Claude's response posted as reply
- Gateway provides monitoring endpoints

---

## v0.2.0 - Conversation Context & History

**Goal:** Add memory and context to conversations.

### Features to Implement

**Conversation Management:**
- [ ] Per-channel conversation history
- [ ] Context window management (token limits)
- [ ] Conversation threading/isolation
- [ ] Clear/reset conversation command

**Database:**
- [ ] SQLite integration for persistence
- [ ] Schema: conversations, messages, users
- [ ] Message history storage
- [ ] Conversation metadata (started, last_active)

**API Enhancements:**
- [ ] Multi-turn conversations with Claude
- [ ] System prompts per channel/server
- [ ] Temperature and parameter configuration

**Testing:**
- [ ] Database migration tests
- [ ] Conversation flow tests
- [ ] Context window tests

**Tasks:**
1. Add `sqlx` dependency
2. Create database schema
3. Implement conversation store
4. Update handler to use history
5. Add context trimming logic
6. Add `/clear` command
7. Write integration tests

---

## v0.3.0 - Streaming Responses

**Goal:** Real-time streaming for better UX.

### Features to Implement

**Streaming:**
- [ ] Claude API streaming support
- [ ] Discord message updates (edit as tokens arrive)
- [ ] Typing indicators
- [ ] Partial response handling

**UX Improvements:**
- [ ] Show "thinking..." indicator
- [ ] Progressive message updates
- [ ] Handle stream errors gracefully
- [ ] Stream cancellation on user request

**Technical:**
- [ ] Server-Sent Events (SSE) from Claude
- [ ] WebSocket streaming to gateway clients
- [ ] Buffering and batching strategy

**Testing:**
- [ ] Mock streaming responses
- [ ] Error handling during streams
- [ ] Cancellation tests

**Tasks:**
1. Update Claude client for streaming
2. Implement streaming response parser
3. Add Discord message editing
4. Handle connection interruptions
5. Add stream progress to gateway
6. Write streaming tests

---

## v0.4.0 - Tool Use & Function Calling

**Goal:** Enable Claude to use tools and call functions.

### Features to Implement

**Tool Framework:**
- [ ] Tool definition interface
- [ ] Tool registration system
- [ ] Tool execution framework
- [ ] Result formatting and injection

**Built-in Tools:**
- [ ] Web search
- [ ] Calculator
- [ ] Time/date queries
- [ ] Weather lookup
- [ ] URL content fetching

**Discord Tools:**
- [ ] Channel management
- [ ] Role assignment
- [ ] User information lookup
- [ ] Server statistics

**Safety:**
- [ ] Permission system for tools
- [ ] Rate limiting per tool
- [ ] Audit logging
- [ ] User confirmation for destructive operations

**Testing:**
- [ ] Mock tool execution
- [ ] Permission tests
- [ ] Tool error handling
- [ ] Multi-tool conversation tests

**Tasks:**
1. Design tool trait/interface
2. Implement tool registry
3. Update Claude client for tool use
4. Add tool execution loop
5. Build initial tool set
6. Add permission framework
7. Write tool integration tests

---

## v0.5.0 - Agent Framework

**Goal:** Multi-agent system with specialized behaviors.

### Features to Implement

**Agent System:**
- [ ] Agent trait/interface
- [ ] Agent registry and lifecycle
- [ ] Agent-to-agent communication
- [ ] Shared context/state management

**Agent Types:**
- [ ] Conversation agent (default)
- [ ] Code review agent
- [ ] Debugging agent
- [ ] Research agent
- [ ] Creative writing agent

**Orchestration:**
- [ ] Agent selection logic
- [ ] Multi-agent collaboration
- [ ] Agent handoff protocol
- [ ] Conflict resolution

**Configuration:**
- [ ] Per-agent prompts and parameters
- [ ] Agent-specific tool access
- [ ] Dynamic agent loading
- [ ] Hot-reload capabilities

**Testing:**
- [ ] Agent lifecycle tests
- [ ] Multi-agent integration tests
- [ ] State sharing tests
- [ ] Handoff tests

**Tasks:**
1. Design agent architecture
2. Create base agent trait
3. Implement agent registry
4. Build conversation agent
5. Add agent selection logic
6. Create specialized agents
7. Add coordination layer
8. Write agent tests

---

## v0.6.0 - Advanced Gateway Features

**Goal:** Rich monitoring and control via gateway.

### Features to Implement

**Admin Commands:**
- [ ] Start/stop bot via WebSocket
- [ ] Change configuration at runtime
- [ ] View active conversations
- [ ] Force disconnect users
- [ ] Clear conversation history

**Monitoring:**
- [ ] Real-time message metrics
- [ ] API usage statistics
- [ ] Error rate tracking
- [ ] Performance metrics
- [ ] Health checks with details

**WebSocket Protocol:**
- [ ] Bidirectional streaming
- [ ] Subscription model
- [ ] Event broadcasting
- [ ] Authentication/authorization

**Dashboard (Future):**
- [ ] Web UI for monitoring
- [ ] Conversation viewer
- [ ] Configuration editor
- [ ] Metrics visualization

**Testing:**
- [ ] WebSocket protocol tests
- [ ] Admin command tests
- [ ] Authentication tests
- [ ] Metrics accuracy tests

**Tasks:**
1. Expand WebSocket protocol
2. Add admin command handlers
3. Implement metrics collection
4. Add authentication layer
5. Create subscription system
6. Build monitoring endpoints
7. Write integration tests

---

## v0.7.0 - Channel & Permission Management

**Goal:** Fine-grained control over bot behavior.

### Features to Implement

**Channel Configuration:**
- [ ] Per-channel enable/disable
- [ ] Channel-specific system prompts
- [ ] Channel-specific tool access
- [ ] Channel-specific rate limits

**Permission System:**
- [ ] Role-based access control
- [ ] User allowlist/blocklist
- [ ] Command permissions
- [ ] Tool use permissions

**Admin Features:**
- [ ] Configuration via commands
- [ ] Permission management UI
- [ ] Audit logging
- [ ] Usage reporting

**Testing:**
- [ ] Permission enforcement tests
- [ ] Channel config tests
- [ ] Access control tests
- [ ] Audit log tests

**Tasks:**
1. Design permission model
2. Add channel configuration storage
3. Implement access control
4. Add admin commands
5. Create audit logging
6. Write permission tests

---

## v0.8.0 - Performance & Scalability

**Goal:** Optimize for production scale.

### Features to Implement

**Performance:**
- [ ] Response caching
- [ ] Message batching
- [ ] Connection pooling
- [ ] Lazy loading

**Rate Limiting:**
- [ ] Per-user rate limits
- [ ] Per-channel rate limits
- [ ] Adaptive rate limiting
- [ ] Queue management

**Resource Management:**
- [ ] Memory usage optimization
- [ ] Database connection pooling
- [ ] API request batching
- [ ] Graceful degradation

**Monitoring:**
- [ ] Prometheus metrics
- [ ] OpenTelemetry tracing
- [ ] Performance profiling
- [ ] Resource usage alerts

**Testing:**
- [ ] Load tests
- [ ] Stress tests
- [ ] Memory leak tests
- [ ] Performance benchmarks

**Tasks:**
1. Add caching layer
2. Implement rate limiting
3. Optimize database queries
4. Add metrics collection
5. Set up profiling
6. Write performance tests

---

## v0.9.0 - Plugin System

**Goal:** Extensibility through plugins.

### Features to Implement

**Plugin Framework:**
- [ ] Plugin trait/interface
- [ ] Plugin discovery and loading
- [ ] Plugin lifecycle management
- [ ] Sandboxing/isolation

**Plugin API:**
- [ ] Message hooks
- [ ] Command registration
- [ ] Tool registration
- [ ] Configuration access

**Built-in Plugins:**
- [ ] Reminder plugin
- [ ] Poll/voting plugin
- [ ] Moderation plugin
- [ ] Games plugin

**Plugin Management:**
- [ ] Enable/disable at runtime
- [ ] Plugin configuration
- [ ] Dependency management
- [ ] Version compatibility

**Testing:**
- [ ] Plugin loading tests
- [ ] Plugin isolation tests
- [ ] Hook execution tests
- [ ] Plugin conflict tests

**Tasks:**
1. Design plugin architecture
2. Create plugin trait
3. Implement plugin loader
4. Add plugin hooks
5. Build sample plugins
6. Add management commands
7. Write plugin tests

---

## v1.0.0 - Production Ready

**Goal:** Stable, documented, production-grade release.

### Features to Complete

**Documentation:**
- [ ] Complete API documentation
- [ ] Architecture guide
- [ ] Plugin development guide
- [ ] Deployment guide
- [ ] Operations runbook

**Deployment:**
- [ ] Docker images
- [ ] Kubernetes manifests
- [ ] CI/CD pipeline
- [ ] Automated releases

**Reliability:**
- [ ] Comprehensive error recovery
- [ ] Auto-reconnection logic
- [ ] Data backup/restore
- [ ] Disaster recovery plan

**Security:**
- [ ] Security audit
- [ ] Dependency scanning
- [ ] Secret management
- [ ] Rate limiting hardening

**Testing:**
- [ ] 90%+ code coverage
- [ ] Integration test suite
- [ ] E2E test automation
- [ ] Performance regression tests

**Tasks:**
1. Complete documentation
2. Security hardening
3. Production deployment setup
4. Monitoring and alerting
5. Backup and recovery
6. Final testing
7. Release preparation

---

## Future Considerations (v2.0+)

**Advanced AI Features:**
- Multi-modal support (images, files)
- Fine-tuned models
- Retrieval-augmented generation (RAG)
- Custom embeddings

**Platform Expansion:**
- Slack integration
- Telegram support
- Matrix/Element support
- Generic webhook interface

**Enterprise Features:**
- SSO/SAML authentication
- Multi-tenancy
- Usage quotas
- Billing integration

**Developer Experience:**
- Hot reload
- Interactive debugging
- Plugin marketplace
- Low-code plugin builder

---

## Contributing to Roadmap

See `CONTRIBUTING.md` for how to propose new features or pick up roadmap items.

**Priority:** Focus on v0.2.0 → v0.3.0 → v0.4.0 in order. Each version builds on the previous.

**TDD Required:** All features must follow RED-GREEN-REFACTOR. No exceptions.
