# ADR-002: Strict Test-Driven Development

## Status

Accepted

Date: 2026-01-26

## Context

We need to establish a development methodology that ensures code quality, prevents regressions, and serves as living documentation. The project requires:

- **High reliability**: Bot must handle errors gracefully
- **Maintainability**: Easy to refactor and extend
- **Documentation**: Tests serve as usage examples
- **Regression prevention**: Changes don't break existing features
- **Confidence**: Know that code works before deployment

Alternatives considered:
- **Test-after**: Write code first, tests later (often skipped)
- **No tests**: Rely on manual testing (error-prone, not scalable)
- **TDD**: Write tests first, code second (slower initially, better long-term)
- **BDD**: Behavior-driven, more overhead for this project

## Decision

We will use **strict Test-Driven Development (TDD)** following the RED-GREEN-REFACTOR cycle for all features.

**Process:**

1. **RED**: Write a failing test first
   - Test doesn't compile (type doesn't exist), OR
   - Test compiles but fails assertion
   - Run `cargo test` - must fail

2. **GREEN**: Write minimal code to pass
   - Just enough to make test pass
   - No premature optimization
   - Run `cargo test` - must pass

3. **REFACTOR**: Improve while tests stay green
   - Add documentation
   - Extract functions
   - Improve naming
   - Run `cargo test` - must still pass

**Rules:**
- No production code without a failing test first
- Tests must be clear and focused
- Each test tests one thing
- Integration tests for workflows
- Mock external dependencies (HTTP, Discord API)

## Consequences

### Positive

- **Quality**: Catches bugs before they reach production
- **Design**: Forces thinking about API before implementation
- **Documentation**: Tests show how to use the code
- **Refactoring confidence**: Safe to change code with test coverage
- **Regression prevention**: Old bugs don't come back
- **Code review**: Tests clarify intent

### Negative

- **Initial slowdown**: Writing tests first takes more time upfront
- **Learning curve**: Team must understand TDD principles
- **Discipline required**: Easy to skip RED phase under pressure
- **Test maintenance**: Tests need updates when requirements change

### Neutral

- **Test coverage**: High coverage by default, but doesn't guarantee correctness
- **Mocking complexity**: Some dependencies hard to mock (minimized with design)

## Notes

This decision was made during project inception and followed throughout all 5 phases of MVP development, resulting in:
- 48 tests (44 unit + 4 integration)
- 100% test success rate
- Zero runtime surprises during development

The strict adherence to RED-GREEN-REFACTOR proved valuable - every bug was caught by failing tests before code was written to fix it.

## References

- [Test-Driven Development by Example - Kent Beck](https://www.amazon.com/Test-Driven-Development-Kent-Beck/dp/0321146530)
- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [TDD in Rust Tutorial](https://github.com/rust-lang/book/blob/main/src/ch11-01-writing-tests.md)
