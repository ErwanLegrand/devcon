# Plan: Security — Warn When Container Runs as Root With No remoteUser Configured

## Phase 1: Root User Detection

- [ ] Task: Implement `fn detect_root_user(provider: &dyn Provider, name: &str) -> io::Result<bool>`
    - [ ] Run `docker inspect --format={{.Config.User}} <name>` (or podman equivalent)
    - [ ] Return `true` if user field is empty, `"root"`, or `"0"`
    - [ ] In `src/devcontainers/mod.rs` or a new `src/devcontainers/user_check.rs`
    - [ ] `cargo build` must pass
- [ ] Task: Wire warning into `Devcontainer::run()`
    - [ ] After container is created, before attach, call `detect_root_user`
    - [ ] If root and no `remoteUser` configured, emit `eprintln!` warning
    - [ ] Skip check if `remoteUser` is set or `--no-root-check` flag is present
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(security): warn when container runs as root with no remoteUser set`

## Phase 2: Tests

- [ ] Task: Unit tests for root detection
    - [ ] Empty user string → root (true)
    - [ ] `"root"` → true
    - [ ] `"0"` → true
    - [ ] `"vscode"` → false
    - [ ] Warning suppressed when `remoteUser` is set
    - [ ] Warning suppressed with `--no-root-check`
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): root user detection and warning`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
