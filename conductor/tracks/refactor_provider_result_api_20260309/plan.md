# Plan: Refactor — Provider Trait exec Methods Return Result<()>

## Phase 1: Update Provider Trait Signature

- [ ] Task: Change `exec` and `exec_raw` signatures in `Provider` trait
    - [ ] `fn exec(&self, cmd: String) -> Result<()>` — Err on non-zero exit
    - [ ] `fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<()>`
    - [ ] `cargo build` will fail until all implementations are updated

## Phase 2: Update All Four Implementations

- [ ] Task: Update `Docker::exec` and `Docker::exec_raw`
    - [ ] Return `Err` if exit status is non-zero (include exit code in error)
    - [ ] `cargo build` must pass
- [ ] Task: Update `Podman::exec` and `Podman::exec_raw`
    - [ ] `cargo build` must pass
- [ ] Task: Update `DockerCompose::exec` and `DockerCompose::exec_raw`
    - [ ] `cargo build` must pass
- [ ] Task: Update `PodmanCompose::exec` and `PodmanCompose::exec_raw`
    - [ ] `cargo build` must pass

## Phase 3: Update Callers

- [ ] Task: Simplify `exec_hook` and `exec_host_hook` in `src/devcontainers/mod.rs`
    - [ ] Remove `Ok(false)` checks; use `?` propagation
    - [ ] `cargo build` must pass
    - [ ] Commit: `refactor(provider): exec/exec_raw return Result<()> for cleaner error propagation`

## Phase 4: Tests and Quality Gate

- [ ] Task: Update tests for new return types
    - [ ] MockProvider updated to return `Result<()>`
    - [ ] Tests verify `Err` on failure, `Ok(())` on success
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): updated exec/exec_raw return type tests`
- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
