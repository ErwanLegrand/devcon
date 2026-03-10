# Plan: Security — Redact Sensitive Values From Logged Commands

## Phase 1: Redaction in print_command

- [ ] Task: Implement `redact_args(args: &[impl AsRef<str>]) -> Vec<String>`
    - [ ] In `src/provider/mod.rs`, detect `--env KEY=VALUE` / `-e KEY=VALUE` / `--env=KEY=VALUE`
    - [ ] Return new vec with VALUE replaced by `***`
    - [ ] `cargo build` must pass
- [ ] Task: Use `redact_args` in `print_command`
    - [ ] Apply redaction before printing; pass original args to actual command
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(security): redact env var values in printed command output`

## Phase 2: Tests

- [ ] Task: Unit tests for `redact_args`
    - [ ] `--env FOO=bar` → `--env FOO=***`
    - [ ] `--env=FOO=bar` → `--env=FOO=***`
    - [ ] `-e FOO=bar` (separate arg) → `-e FOO=***`
    - [ ] `--env FOO` (no value, passthrough) → unchanged
    - [ ] Non-env args → unchanged
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): env var redaction in print_command`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
