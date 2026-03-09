# Plan: Security — Sanitise Container Names and Label Values Used in Shell Commands

## Phase 1: Validation Functions

- [ ] Task: Strengthen `safe_name()` validation
    - [ ] In `src/devcontainers/mod.rs` (or a new `src/devcontainers/names.rs`), validate that
          `safe_name()` output matches `^[a-zA-Z0-9][a-zA-Z0-9_.-]*$`
    - [ ] Return error if workspace name cannot produce a safe container name
    - [ ] `cargo build` must pass
- [ ] Task: Add `validate_env_key(key: &str) -> Result<()>` and `validate_env_value(val: &str) -> Result<()>`
    - [ ] Key: must match `^[A-Za-z_][A-Za-z0-9_]*$`
    - [ ] Value: must not contain null bytes
    - [ ] Call both in `Devcontainer::load()` or `run()` for `remoteEnv` and `containerEnv`
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(security): validate container names and env var keys/values`

## Phase 2: Tests

- [ ] Task: Unit tests for all three validators
    - [ ] `safe_name` valid: `myproject`, `my-project.v2`
    - [ ] `safe_name` invalid: starts with `-`, contains `$`, contains space
    - [ ] `validate_env_key` valid: `FOO`, `_BAR`, `MY_VAR_1`
    - [ ] `validate_env_key` invalid: `1FOO`, `FOO-BAR`, `FOO BAR`
    - [ ] `validate_env_value` with null byte → error
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): container name and env var validation`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
