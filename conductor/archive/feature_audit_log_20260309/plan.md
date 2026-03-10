# Plan: Feature — Structured Audit Log

## Phase 1: Audit Log Infrastructure

- [ ] Task: Create `src/audit.rs`
    - [ ] Define `AuditEvent` enum and `AuditLogger` struct
    - [ ] Implement `AuditLogger::new()` — opens/creates log file with mode 0o600
    - [ ] Implement `AuditLogger::log(event: &AuditEvent) -> io::Result<()>`
    - [ ] Implement JSON serialization for each event type (use `serde_json`)
    - [ ] Handle write failures gracefully (warn to stderr, continue)
    - [ ] `cargo build` must pass
- [ ] Task: Wire `AuditLogger` into `Devcontainer`
    - [ ] Create logger in `run()` and `rebuild()`
    - [ ] Emit `container_start`, `container_stop`, `hook_executed`, `command_run` events
    - [ ] Respect `--no-audit-log` flag
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(audit): structured lifecycle audit log to XDG data dir`

## Phase 2: Tests

- [ ] Task: Unit tests for audit log
    - [ ] Log entry format is valid NDJSON
    - [ ] Env var values are redacted in `command_run` events
    - [ ] Write failure does not propagate as error
    - [ ] Log file created with mode 0o600
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(audit): log entry format and write failure handling`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
