# Plan: Security — Require User Confirmation Before Running initializeCommand on Host

## Phase 1: Confirmation Prompt

- [ ] Task: Add `--trust` flag to `start` and `rebuild` commands
    - [ ] In `src/commands/start.rs` and `rebuild.rs`, add `--trust` / `-y` CLI flag via `clap`
    - [ ] Pass trust flag down to `Devcontainer::run()` / `rebuild()` via a settings field
          or directly as a parameter
    - [ ] `cargo build` must pass
- [ ] Task: Implement prompt in `exec_host_hook` caller
    - [ ] Before executing `initializeCommand`, if `--trust` is absent, print command to stderr
          and prompt `"Run initializeCommand on host? [y/N] "` reading from stdin
    - [ ] Default to No on empty input or non-`y` response
    - [ ] Abort with exit code 1 and message `"initializeCommand declined by user"` on No
    - [ ] Skip prompt and log notice when `--trust` is set
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(security): prompt before running initializeCommand on host`

## Phase 2: Tests

- [ ] Task: Unit tests for confirmation logic
    - [ ] `--trust` flag: prompt skipped, hook runs
    - [ ] `y` input: hook runs
    - [ ] `N` / empty input: returns error
    - [ ] Prompt text includes the command being run
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): initializeCommand confirmation prompt`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
