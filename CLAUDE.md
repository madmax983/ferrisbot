# CLAUDE.md - Development Guide for AI Assistants

## Project Overview

**Ferrisbot** is a Discord bot powered by Claude AI, built with Rust using strict Test-Driven Development (TDD) principles.

**Current Status:** v0.1.0 - MVP Foundation Complete

### What We're Building

**MVP (v0.1.0 - COMPLETE):** A solid foundation providing:
- Discord message handling with filtering
- Claude API integration with error handling
- HTTP/WebSocket gateway for monitoring
- Basic chat: User → Discord → Claude → Reply

**Post-MVP (v0.2.0+):** Extending the foundation with:
- Agent framework and tool use
- Streaming responses
- Conversation memory/context
- Multi-agent conversations
- Database persistence
- Advanced admin controls
- Plugin system

### Architecture

```
Discord Bot (Serenity) ←→ Claude Client (Reqwest) ←→ Anthropic API
           ↓
    HTTP Gateway (Axum)
    - /health endpoint
    - /ws WebSocket
```

---

## TDD Methodology - CRITICAL

This project **REQUIRES** strict Test-Driven Development. Every feature must follow RED-GREEN-REFACTOR.

### RED Phase - Write Failing Test First

**BEFORE writing any implementation code, write a test that fails.**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature_doesnt_exist_yet() {
        // RED: This will fail because NewFeature doesn't exist
        let feature = NewFeature::new("test");
        assert_eq!(feature.value(), "test");
    }
}
```

Run the test - it should **FAIL**:
```bash
cargo test
# Expected: compilation error or test failure
```

### GREEN Phase - Write Minimal Code to Pass

**Write ONLY enough code to make the test pass. No more.**

```rust
pub struct NewFeature {
    value: String,
}

impl NewFeature {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
```

Run the test - it should **PASS**:
```bash
cargo test
# Expected: test passes
```

### REFACTOR Phase - Clean Up While Tests Stay Green

**Improve code quality without changing behavior.**

```rust
// Add documentation
/// Represents a feature in the system
#[derive(Debug, Clone)]
pub struct NewFeature {
    value: String,
}

impl NewFeature {
    /// Create a new feature with the given value
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Get the feature value
    pub fn value(&self) -> &str {
        &self.value
    }
}

// Add more tests for edge cases
#[test]
fn test_feature_with_empty_string() {
    let feature = NewFeature::new("");
    assert_eq!(feature.value(), "");
}
```

Run tests again - they should still **PASS**:
```bash
cargo test
# Expected: all tests pass
```

---

## Code Quality Requirements

### ALWAYS Run Before Committing

```bash
# 1. Format code
cargo fmt

# 2. Check for issues
cargo clippy --all-targets

# 3. Run all tests
cargo test

# All three must pass before committing!
```

### Clippy Rules

Fix ALL clippy warnings except:
- `dead_code` warnings during development (acceptable for incomplete features)
- Warnings in generated code (rare)

**Zero tolerance for:**
- Unused imports
- Unused variables (use `_` prefix if intentional)
- Complex boolean expressions
- Inefficient string operations

### Formatting

Use **default rustfmt** settings. The project uses:
```toml
# rustfmt.toml (default settings)
edition = "2021"
```

---

## Module Structure

### Core Modules

```
src/
├── lib.rs              # Library root - public API
├── main.rs             # Binary entry - application startup
├── app.rs              # Orchestration - lifecycle management
├── error.rs            # Error types - FerrisError enum
├── types/              # Domain types - Message, etc.
├── llm/                # LLM integration
│   ├── claude.rs       # Claude API client
│   └── types.rs        # Request/Response types
├── discord/            # Discord integration
│   ├── bot.rs          # Bot wrapper
│   ├── handler.rs      # Event handler
│   └── convert.rs      # Type conversions
└── gateway/            # HTTP/WebSocket
    ├── server.rs       # Axum setup
    ├── routes.rs       # HTTP routes
    └── ws.rs           # WebSocket handler
```

### Adding a New Module

1. **RED:** Write test in `mod.rs`
2. **GREEN:** Create module file
3. **REFACTOR:** Add documentation
4. Export in parent `mod.rs`

Example:
```rust
// src/new_module/mod.rs
pub mod feature;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_works() {
        // RED: Write failing test first
    }
}
```

---

## Common Patterns

### Error Handling

**ALWAYS use `Result<T>` for fallible operations:**

```rust
use crate::error::{Result, FerrisError};

pub fn might_fail(input: &str) -> Result<String> {
    if input.is_empty() {
        return Err(FerrisError::Config("Input cannot be empty".to_string()));
    }
    Ok(input.to_uppercase())
}
```

**NEVER use `.unwrap()` in production code paths:**
```rust
// ❌ BAD
let value = some_option.unwrap();

// ✅ GOOD
let value = some_option.ok_or_else(||
    FerrisError::Config("Value missing".to_string())
)?;
```

### Async/Await

Use Tokio for all async operations:

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Builder Pattern

For complex configuration:

```rust
pub struct Config {
    field1: String,
    field2: u32,
}

pub struct ConfigBuilder {
    field1: String,
    field2: u32,
}

impl ConfigBuilder {
    pub fn field1(mut self, value: impl Into<String>) -> Self {
        self.field1 = value.into();
        self
    }

    pub fn build(self) -> Config {
        Config {
            field1: self.field1,
            field2: self.field2,
        }
    }
}
```

---

## Testing Guidelines

### Test Organization

```rust
// Production code
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Tests in same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use ferrisbot::app::App;

#[tokio::test]
async fn test_full_flow() {
    // Test complete workflows
}
```

### Mock Testing

Use `wiremock` for HTTP mocks:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_with_mock_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // Test against mock_server.uri()
}
```

---

## Git Workflow

### Before Every Commit

```bash
# 1. Format
cargo fmt

# 2. Lint
cargo clippy --all-targets --fix --allow-dirty

# 3. Test
cargo test

# 4. Commit only if all pass
git add .
git commit -m "feat: add new feature

- Added NewFeature struct
- Implemented RED-GREEN-REFACTOR cycle
- All tests passing"
```

### Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat:` - New feature
- `fix:` - Bug fix
- `test:` - Add/update tests
- `refactor:` - Code refactoring
- `docs:` - Documentation
- `chore:` - Maintenance

Example:
```
feat: add message filtering to Discord handler

- Filter out bot messages to prevent loops
- Filter empty messages
- Add tests for filtering logic
- All tests passing (52/52)

Closes #123
```

---

## Adding New Features

### Step-by-Step Process

1. **Plan** - Understand requirements
2. **RED** - Write failing test
3. **GREEN** - Implement minimal code
4. **REFACTOR** - Clean up code
5. **Format** - `cargo fmt`
6. **Lint** - `cargo clippy`
7. **Test** - `cargo test`
8. **Commit** - If all pass

### Example: Adding Rate Limiting

**Step 1: RED - Write Test**
```rust
// src/discord/rate_limit.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_first_message() {
        let limiter = RateLimiter::new(5, Duration::from_secs(60));
        assert!(limiter.check("user123").await);
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_after_limit() {
        let limiter = RateLimiter::new(2, Duration::from_secs(60));
        assert!(limiter.check("user123").await);
        assert!(limiter.check("user123").await);
        assert!(!limiter.check("user123").await); // Should be blocked
    }
}
```

**Step 2: GREEN - Implement**
```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct RateLimiter {
    max_requests: usize,
    window: Duration,
    requests: Mutex<HashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: Mutex::new(HashMap::new()),
        }
    }

    pub async fn check(&self, user_id: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();

        let user_requests = requests.entry(user_id.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside the window
        user_requests.retain(|&time| now.duration_since(time) < self.window);

        if user_requests.len() < self.max_requests {
            user_requests.push(now);
            true
        } else {
            false
        }
    }
}
```

**Step 3: REFACTOR - Add Documentation**
```rust
/// Rate limiter for preventing spam
///
/// Uses a sliding window algorithm to track requests per user.
pub struct RateLimiter {
    // ... fields
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `max_requests` - Maximum requests allowed in the window
    /// * `window` - Time window for rate limiting
    pub fn new(max_requests: usize, window: Duration) -> Self {
        // ...
    }

    /// Check if a request should be allowed
    ///
    /// Returns `true` if the request is allowed, `false` if rate limited.
    pub async fn check(&self, user_id: &str) -> bool {
        // ...
    }
}
```

**Step 4: Verify**
```bash
cargo fmt
cargo clippy
cargo test
```

---

## Debugging

### Enable Logging

```bash
RUST_LOG=debug cargo run
RUST_LOG=ferrisbot::discord=trace cargo run
```

### Common Issues

**Issue: Tests hang**
```bash
# Use timeout
cargo test -- --test-threads=1 --nocapture
```

**Issue: Clippy warnings**
```bash
# Auto-fix
cargo clippy --fix --allow-dirty
```

**Issue: Format differences**
```bash
# Check what would change
cargo fmt -- --check

# Apply formatting
cargo fmt
```

---

## Performance Considerations

### Async Runtime

- Use `Arc` for shared state across tasks
- Use `tokio::spawn` for concurrent tasks
- Use `tokio::select!` for racing tasks

### Memory

- Prefer `&str` over `String` where possible
- Use `Arc<str>` for shared immutable strings
- Clone only when necessary

### Error Handling

- Use `?` operator for propagation
- Match on errors for specific handling
- Log errors with context

---

## Documentation Standards

### Public API

**ALWAYS document public items:**

```rust
/// Calculate the sum of two numbers
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The sum of `a` and `b`
///
/// # Example
/// ```
/// use ferrisbot::math::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Module-Level Docs

```rust
//! Discord integration module
//!
//! This module handles all Discord-related functionality including:
//! - Bot connection and lifecycle
//! - Message event handling
//! - Type conversions
```

---

## CI/CD - GitHub Actions

The project uses GitHub Actions for continuous integration and deployment.

### Workflows

**CI (`.github/workflows/ci.yml`):**

Runs on every push and PR to `main`, `trunk`, `develop`:

1. **Format Check** - `cargo fmt --check`
   - Ensures code is properly formatted
   - Fails if any file needs formatting

2. **Clippy Lint** - `cargo clippy --all-targets --all-features -- -D warnings`
   - Runs Rust linter
   - Fails on ANY warnings (zero tolerance)

3. **Tests** - `cargo test --all-features`
   - Runs on Linux, Windows, macOS
   - All tests must pass on all platforms
   - Includes doc tests

4. **Coverage** - `cargo tarpaulin`
   - Generates code coverage report
   - Uploads to Codecov
   - Does not fail CI (informational)

5. **Build** - `cargo build --release`
   - Verifies release builds succeed
   - Runs on all platforms

6. **Security Audit** - `cargo-audit`
   - Checks for security vulnerabilities
   - Scans dependencies

**All checks must pass before merge.**

**Release (`.github/workflows/release.yml`):**

Triggered by version tags (`v*.*.*`):
- Builds release binaries for:
  - Linux (x86_64)
  - Windows (x86_64)
  - macOS (x86_64 + ARM64)
- Creates GitHub release
- Attaches binaries

**Nightly (`.github/workflows/nightly.yml`):**

Runs daily at 2 AM UTC:
- Tests against Rust nightly
- Checks for outdated dependencies
- Early warning for breaking changes

### Local Pre-Commit Checks

**Always run before committing:**

```bash
# 1. Format
cargo fmt

# 2. Clippy (match CI exactly)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Tests
cargo test --all-features

# 4. Build
cargo build --release
```

**All must succeed before pushing!**

### Setting Up Pre-Commit Hook (Optional)

```bash
# Create .git/hooks/pre-commit
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
set -e

echo "Running pre-commit checks..."

echo "1. Formatting..."
cargo fmt -- --check

echo "2. Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "3. Tests..."
cargo test --all-features --quiet

echo "✅ All checks passed!"
EOF

chmod +x .git/hooks/pre-commit
```

---

## Environment Variables

Required:
- `DISCORD_TOKEN` - Discord bot token
- `ANTHROPIC_API_KEY` - Claude API key

Optional:
- `GATEWAY_PORT` - Gateway port (default: 18789)
- `RUST_LOG` - Logging level (default: info)

**NEVER commit secrets to git!**

---

## Review Checklist

Before submitting code for review:

- [ ] Followed RED-GREEN-REFACTOR for all features
- [ ] All tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation added for public APIs
- [ ] Error handling uses `Result<T>`
- [ ] No `.unwrap()` in production paths
- [ ] Async operations don't block
- [ ] Integration tests added if needed
- [ ] CHANGELOG.md updated (if applicable)

---

## Development Roadmap

This is v0.1.0 - the **MVP foundation**. See `ROADMAP.md` for the full development plan.

**Next priorities:**
1. **v0.2.0** - Conversation context & history
2. **v0.3.0** - Streaming responses
3. **v0.4.0** - Tool use & function calling
4. **v0.5.0** - Agent framework
5. And beyond...

**When adding new features:**
1. Check `ROADMAP.md` for planned architecture
2. Follow TDD for all implementations
3. Update roadmap when completing features
4. Document breaking changes

---

## Resources

### Official Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serenity Docs](https://docs.rs/serenity/)
- [Axum Guide](https://docs.rs/axum/)
- [TDD in Rust](https://doc.rust-lang.org/book/ch11-00-testing.html)

### Project Documentation
- `README.md` - Project overview
- `ROADMAP.md` - Development plan
- `QUICKSTART.md` - Setup guide
- `IMPLEMENTATION_SUMMARY.md` - Architecture details
- `VERIFICATION_CHECKLIST.md` - Deployment checks

---

## Getting Help

1. Check existing tests for examples
2. Review similar modules in the codebase
3. Read inline documentation
4. Check `IMPLEMENTATION_SUMMARY.md` for architecture details
5. Review `ROADMAP.md` for planned features
6. Check `QUICKSTART.md` for setup issues

---

## Philosophy

**We are building a foundation, not a finished product.**

- v0.1.0 MVP proves the architecture works
- Each version adds significant new capabilities
- TDD ensures quality at every step
- Documentation keeps pace with code
- Tests are the specification

**Remember: RED → GREEN → REFACTOR. Always.**

**Test first, code second. No exceptions.**

**This is v0.1.0 of a long journey. Build it right.**
