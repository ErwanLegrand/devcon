# Plan: Fix — Dockerfile Path Resolution Asymmetry Between Docker and Podman

## Phase 1: Shared Resolution Helper

- [ ] Task: Implement `resolve_dockerfile_path` helper
    - [ ] In `src/provider/utils.rs`, add `pub(crate) fn resolve_dockerfile_path(workspace: &Path, build: &Build) -> PathBuf`
    - [ ] Resolves dockerfile relative to context if set, otherwise relative to workspace root
    - [ ] `cargo build` must pass

## Phase 2: Update All Four Providers

- [ ] Task: Update `Docker::build()` to use the helper
    - [ ] Replace inline path construction with `resolve_dockerfile_path`
    - [ ] `cargo build` must pass
- [ ] Task: Update `Podman::build()` to use the helper
    - [ ] `cargo build` must pass
- [ ] Task: Update `DockerCompose::build()` and `PodmanCompose::build()` if applicable
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(provider): unify dockerfile path resolution across all providers`

## Phase 3: Tests

- [ ] Task: Unit tests for `resolve_dockerfile_path`
    - [ ] Relative dockerfile with explicit context → resolved against context
    - [ ] Relative dockerfile without context → resolved against workspace root
    - [ ] Absolute dockerfile path → returned as-is
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): dockerfile path resolution`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
