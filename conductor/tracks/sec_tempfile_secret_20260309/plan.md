# Plan: Security — Fix Compose Override Temp File Permissions and Cleanup

## Phase 1: Fix File Permissions

- [ ] Task: Update `create_compose_override` to use mode 0o600
    - [ ] In `src/provider/utils.rs`, replace `fs::write(path, content)` with
          `OpenOptions::new().write(true).create(true).mode(0o600).open(path)?`
    - [ ] `cargo build` must pass
- [ ] Task: Introduce a `ComposeOverrideGuard` wrapper
    - [ ] Create a newtype `struct ComposeOverrideGuard(PathBuf)` that deletes the file on `Drop`
    - [ ] `create_compose_override` returns `ComposeOverrideGuard` instead of `PathBuf`
    - [ ] Update all callers in `docker_compose.rs` and `podman_compose.rs` to hold the guard
          for the duration of the provider call
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(security): create compose override with 0o600 perms and auto-cleanup guard`

## Phase 2: Tests

- [ ] Task: Write unit tests for temp file security properties
    - [ ] Verify created file has mode 0o600 (read file metadata after creation)
    - [ ] Verify file is removed after `ComposeOverrideGuard` is dropped
    - [ ] Verify file is removed even if the block exits early (panic-safe Drop)
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): verify compose override file perms and cleanup`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
