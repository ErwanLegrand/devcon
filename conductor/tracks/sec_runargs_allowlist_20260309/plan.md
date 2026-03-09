# Plan: Security — Validate runArgs Against Allowlist

## Phase 1: Allowlist/Denylist Implementation

- [ ] Task: Create `src/devcontainers/run_args.rs`
    - [ ] Define `ALLOWED_FLAGS: &[&str]` constant with safe flag prefixes
    - [ ] Define `DENIED_FLAGS: &[&str]` constant with privilege-escalating flags
    - [ ] Implement `fn validate_run_args(args: &[String]) -> Result<(), String>`
    - [ ] `cargo build` must pass
- [ ] Task: Wire validation into `Devcontainer::run()` and `rebuild()`
    - [ ] Call `validate_run_args` before passing `runArgs` to providers
    - [ ] Abort with descriptive error if denied flag is present
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(security): validate runArgs against privilege-escalation denylist`

## Phase 2: Tests

- [ ] Task: Write unit tests for `validate_run_args`
    - [ ] Allowed flags: `--env FOO=bar`, `--label`, `--network bridge` — all pass
    - [ ] Denied flags: `--privileged`, `--cap-add ALL`, `--device /dev/sda` — all error
    - [ ] Unknown flags: warn but do not error
    - [ ] Empty args list: passes without error
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): cover runArgs allowlist/denylist validation`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
