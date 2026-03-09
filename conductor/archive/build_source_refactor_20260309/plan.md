# Plan: build_source_for Refactor — Replace bool Parameter with Explicit Enum

## Phase 1: Refactor

- [x] Task: Replace `build_source_for(…, bool)` with two dedicated helpers
    - [x] In `src/devcontainers/mod.rs`, rename/split into:
        - `fn docker_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource>` — uses `directory.join(dockerfile)` directly
        - `fn podman_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource>` — uses `directory.join(".devcontainer").join(dockerfile)`
    - [x] Update call sites in `build_provider` accordingly
    - [x] Delete `build_source_for`
    - [x] `cargo build` must pass
    - [x] `cargo clippy --all-targets -- -D warnings` must pass
    - [x] Commit: `refactor(devcontainer): replace build_source_for bool param with two typed helpers`

## Phase 2: Quality Gate

- [x] Task: Run full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
