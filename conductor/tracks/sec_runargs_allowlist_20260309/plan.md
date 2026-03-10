# Plan: Security — Validate runArgs Against Allowlist

## Phase 1: Allowlist/Denylist Implementation

- [x] Task: Create `src/devcontainers/run_args.rs`
    - [x] Define `ALLOWED_FLAGS: &[&str]` constant with safe flag prefixes
    - [x] Define `DENIED_FLAGS: &[&str]` constant with privilege-escalating flags
    - [x] Implement `fn validate_run_args(args: &[String]) -> Result<(), String>`
    - [x] `cargo build` must pass
- [x] Task: Wire validation into `Devcontainer::run()` and `rebuild()`
    - [x] Call `validate_run_args` before passing `runArgs` to providers
    - [x] Abort with descriptive error if denied flag is present
    - [x] `cargo build` must pass
    - [x] Commit: `feat(security): validate runArgs against privilege-escalation denylist`

## Phase 2: Tests

- [x] Task: Write unit tests for `validate_run_args`
    - [x] Allowed flags: `--env FOO=bar`, `--label`, `--network bridge` — all pass
    - [x] Denied flags: `--privileged`, `--cap-add ALL`, `--device /dev/sda` — all error
    - [x] Unknown flags: warn but do not error
    - [x] Empty args list: passes without error
    - [x] `cargo test` must pass
    - [x] Commit: `test(security): cover runArgs allowlist/denylist validation`

## Phase 3: Quality Gate

- [x] Task: Full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
