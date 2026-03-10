# Plan: Security — Fix Compose Override Temp File Permissions and Cleanup

## Phase 1: Fix File Permissions

- [x] Task: Update `create_compose_override` to use mode 0o600
    - [x] In `src/provider/utils.rs`, replace `fs::write(path, content)` with
          `OpenOptions::new().write(true).create(true).mode(0o600).open(path)?`
    - [x] `cargo build` must pass
- [x] Task: Introduce a `ComposeOverrideGuard` wrapper
    - [x] Create a newtype `struct ComposeOverrideGuard(PathBuf)` that deletes the file on `Drop`
    - [x] `create_compose_override` returns `ComposeOverrideGuard` instead of `PathBuf`
    - [x] Update all callers in `docker_compose.rs` and `podman_compose.rs` to hold the guard
          for the duration of the provider call
    - [x] `cargo build` must pass
    - [x] Commit: `fix(security): create compose override with 0o600 perms and auto-cleanup guard`

## Phase 2: Tests

- [x] Task: Write unit tests for temp file security properties
    - [x] Verify created file has mode 0o600 (read file metadata after creation)
    - [x] Verify file is removed after `ComposeOverrideGuard` is dropped
    - [x] `cargo test` must pass
    - [x] Commit: `test(security): verify compose override file perms and cleanup`

## Phase 3: Quality Gate

- [x] Task: Full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
