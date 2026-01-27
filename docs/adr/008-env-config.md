# ADR-008: Environment-Based Configuration

## Status

Accepted

Date: 2026-01-26

## Context

We need a configuration strategy for:
- API keys (Discord token, Claude key)
- Runtime settings (gateway port)
- Environment-specific values (dev vs prod)

Requirements:
- **Security**: No secrets in code or git
- **Simplicity**: Easy to configure
- **12-factor**: Follow cloud-native principles
- **Defaults**: Sensible defaults where possible

Alternatives considered:
- **Config files**: TOML/YAML (can be committed with secrets)
- **Environment variables**: Standard, secure (our choice)
- **Command-line args**: Good for overrides, not secrets
- **Configuration service**: Overkill for MVP

## Decision

We will use **environment variables** for all configuration, loaded via `std::env`.

Implementation:
```rust
pub struct AppConfig {
    pub discord_token: String,
    pub claude_api_key: String,
    pub gateway_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| FerrisError::Config("DISCORD_TOKEN not set".into()))?;

        let claude_api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| FerrisError::Config("ANTHROPIC_API_KEY not set".into()))?;

        let gateway_port = env::var("GATEWAY_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        Ok(Self { discord_token, claude_api_key, gateway_port })
    }
}
```

Required variables:
- `DISCORD_TOKEN` - Discord bot token
- `ANTHROPIC_API_KEY` - Claude API key

Optional variables:
- `GATEWAY_PORT` - HTTP gateway port (default: 18789)
- `RUST_LOG` - Logging level (default: info)

## Consequences

### Positive

- **Security**: Secrets not in code or git
- **12-factor**: Follows cloud-native best practices
- **Container-friendly**: Easy to configure in Docker/K8s
- **CI/CD**: Easy to set in GitHub Actions
- **Separation**: Dev/staging/prod use same code, different config
- **No files**: No config file management
- **Standard**: Rust ecosystem convention

### Negative

- **Discoverability**: Must document required variables
- **Type safety**: No compile-time checking of config
- **Defaults**: Need to handle missing optional vars
- **Complex config**: Environment variables don't support nesting well

### Neutral

- **Local development**: Need .env file or export commands
- **Documentation**: Must keep docs updated

## Notes

We explicitly DON'T use a config library (like `config` or `figment`) for MVP because:
- Simple needs (3 variables)
- No nested configuration
- Avoid dependency bloat

For v0.2.0+, may add .env file support via `dotenv` crate for local development convenience.

Error messages guide users:
```
Error: Configuration error: DISCORD_TOKEN not set

Required environment variables:
  DISCORD_TOKEN - Discord bot token
  ANTHROPIC_API_KEY - Claude API key

Optional environment variables:
  GATEWAY_PORT - Gateway port (default: 18789)
```

Security note: `.gitignore` includes `.env` to prevent accidental commits.

## References

- [The Twelve-Factor App - Config](https://12factor.net/config)
- [Rust env Module](https://doc.rust-lang.org/std/env/)
- Related: ADR-010 (MVP) - Keeping it simple
