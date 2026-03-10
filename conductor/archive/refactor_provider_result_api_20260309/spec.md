# Spec: Refactor — Provider Trait Methods Return Result<()> Not Result<bool>

## Problem

Several `Provider` trait methods return `Result<bool>` where `true` = success and `false` =
failure. This is an anti-pattern: `Result` already encodes failure as `Err`. The `bool` return
creates ambiguity (is `Ok(false)` a soft failure? an error? unimplemented?) and forces callers
to write awkward double checks:

```rust
if !provider.exec(cmd)? { return Err(...) }
```

Methods affected: `exec`, `exec_raw`, `exists`, `running` (and indirectly all lifecycle hooks).

## Goal

- `exists()` and `running()`: return `Result<bool>` (bool semantics: presence/absence, not error).
- `exec()`, `exec_raw()`: return `Result<ExitStatus>` or `Result<()>` with proper error on non-zero.
- Lifecycle hook callers: simplified to `?` propagation.

## Functional Requirements

- FR-001: `exec()` and `exec_raw()` return `Result<()>`, returning `Err` on non-zero exit.
  The non-zero exit code is included in the error message.
- FR-002: `exists()` and `running()` keep `Result<bool>` — these query state, not execute.
- FR-003: `exec_hook` and `exec_host_hook` simplified to `?` propagation.
- FR-004: All four provider implementations updated.
- FR-005: All callers in `src/devcontainers/mod.rs` updated.
- FR-006: All existing tests pass with updated signatures.
- FR-007: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Changing `Provider::build`, `create`, `start`, `stop`, `restart`, `attach`, `rm`, `cp`
  signatures (evaluate separately).
