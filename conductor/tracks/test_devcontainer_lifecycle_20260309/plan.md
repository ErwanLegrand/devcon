# Plan: Tests — Devcontainer Lifecycle run() and rebuild()

## Phase 1: Extend MockProvider

- [ ] Task: Audit existing MockProvider in `src/devcontainers/mod.rs`
    - [ ] Verify MockProvider implements all Provider trait methods
    - [ ] Add call recording for `build`, `create`, `start`, `attach` if missing
    - [ ] `cargo build` must pass

## Phase 2: Happy Path Tests

- [ ] Task: Test `run()` happy path with all hooks
    - [ ] Config with all six hooks set to simple commands, MockProvider returns success
    - [ ] Verify all hooks called in correct order
    - [ ] `cargo test` must pass
- [ ] Task: Test `run()` with image-based config (no dockerfile)
    - [ ] Verify `build` is not called, `create` is called with image
    - [ ] `cargo test` must pass
- [ ] Task: Test `run()` with no hooks set
    - [ ] Verify exec not called, lifecycle completes
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): lifecycle happy path tests`

## Phase 3: Hook Failure Tests

- [ ] Task: Test each hook failure causes run() to abort
    - [ ] Each of the six hooks failing → `run()` returns `Err` with hook name
    - [ ] Subsequent hooks are not called after a failure
    - [ ] `cargo test` must pass
- [ ] Task: Test `rebuild()` happy path and failure
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): lifecycle hook failure propagation tests`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate and coverage check
    - [ ] `cargo test`
    - [ ] `cargo llvm-cov --lib` — verify `mod.rs` coverage ≥80%
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
