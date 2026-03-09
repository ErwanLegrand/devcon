# Plan: build_source_for Refactor — Replace bool Parameter with Explicit Enum

## Phase 1: Refactor

- [ ] Task: Replace `build_source_for(…, bool)` with two dedicated helpers
    - [ ] In `src/devcontainers/mod.rs`, rename/split into:
        - `fn docker_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource>` — uses `directory.join(dockerfile)` directly
        - `fn podman_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource>` — uses `directory.join(".devcontainer").join(dockerfile)`
    - [ ] Update call sites in `build_provider` accordingly
    - [ ] Delete `build_source_for`
    - [ ] `cargo build` must pass
    - [ ] `cargo clippy --all-targets -- -D warnings` must pass
    - [ ] Commit: `refactor(devcontainer): replace build_source_for bool param with two typed helpers`

## Phase 2: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
