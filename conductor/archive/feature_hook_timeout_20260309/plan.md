# Plan: Feature — Configurable Timeout for Lifecycle Hooks

## Phase 1: Config Field and CLI Flag

- [ ] Task: Add `hook_timeout_seconds: Option<u32>` to `Config`
    - [ ] Parse from `devcontainer.json` field `"hookTimeoutSeconds"`
    - [ ] `cargo build` must pass
- [ ] Task: Add `--hook-timeout <seconds>` CLI flag to `start` and `rebuild`
    - [ ] Overrides config value when set
    - [ ] Pass through to `Devcontainer` at runtime
    - [ ] `cargo build` must pass

## Phase 2: Timeout Implementation

- [ ] Task: Implement timeout wrapper for `exec_host_hook`
    - [ ] Spawn process in a thread; join with timeout duration
    - [ ] On timeout: send SIGTERM, wait 5s, send SIGKILL
    - [ ] Return `Err` with timeout message including hook name and duration
    - [ ] `cargo build` must pass
- [ ] Task: Implement timeout for in-container hooks via `exec_hook`
    - [ ] Use `--timeout` flag if runtime supports it, else thread wrapper
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(hooks): configurable timeout for all lifecycle hooks`

## Phase 3: Tests

- [ ] Task: Unit tests for timeout behaviour
    - [ ] Hook completing within timeout → success
    - [ ] Hook that sleeps past timeout → timeout error with correct message
    - [ ] No timeout configured → no timeout applied (backward compat)
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(hooks): hook timeout coverage`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
