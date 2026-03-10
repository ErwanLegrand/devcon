# Plan: Tests — Devcontainer Lifecycle run() and rebuild()

## Phase 1: Extend MockProvider

- [x] Task: Audit existing MockProvider in `src/devcontainers/mod.rs`
    - [x] Verify MockProvider implements all Provider trait methods
    - [x] Add call recording: build_calls, stop_calls, rm_calls (RefCell<u32>)
    - [x] Add exists_result field and with_existing() constructor
    - [x] `cargo build` must pass

## Phase 2: Happy Path Tests

- [x] Task: Test `run()` happy path with all hooks
    - [x] Config with all five hooks set, MockProvider returns success
    - [x] Verify run() completes without error
- [x] Task: Test `run()` with no hooks set
    - [x] Verify lifecycle completes, exec not called for hooks
- [x] Task: Test `run()` when container does not exist (build path)
    - [x] `cargo test` must pass
    - [x] Commit: `test(devcontainer): lifecycle run() and rebuild() coverage`

## Phase 3: Hook Failure Tests

- [x] Task: Test each hook failure causes run() to abort
    - [x] postCreateCommand failure → Err containing "postCreateCommand"
    - [x] postStartCommand failure → Err containing "postStartCommand"
    - [x] onCreateCommand failure → Err containing "onCreateCommand"
    - [x] updateContentCommand failure → Err containing "updateContentCommand"
    - [x] postAttachCommand failure → Err surfaced
- [x] Task: Test rebuild() happy path (exists=true and exists=false)
    - [x] `cargo test` must pass — 109 tests passing

## Phase 4: Quality Gate

- [x] Task: Full quality gate
    - [x] `cargo test` — 109 tests pass
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
