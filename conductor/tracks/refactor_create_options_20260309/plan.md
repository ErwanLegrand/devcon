# Plan: Refactor — Extract ContainerOptions Struct

## Phase 1: Define ContainerOptions

- [ ] Task: Create `src/devcontainers/options.rs`
    - [ ] Define `pub(crate) struct ContainerOptions` with all relevant fields
    - [ ] Implement `ContainerOptions::from_config(config: &Config, name: String) -> Self`
    - [ ] `cargo build` must pass

## Phase 2: Update run() and rebuild()

- [ ] Task: Build `ContainerOptions` in `Devcontainer::run()` and `rebuild()`
    - [ ] Replace inline argument assembly with `ContainerOptions::from_config`
    - [ ] `cargo build` must pass

## Phase 3: Update Provider Trait and Implementations

- [ ] Task: Update `Provider::create()` to accept `&ContainerOptions`
    - [ ] Update trait definition and all four implementations
    - [ ] `cargo build` must pass
    - [ ] Commit: `refactor(devcontainer): extract ContainerOptions struct for provider calls`

## Phase 4: Tests and Quality Gate

- [ ] Task: Unit tests for `ContainerOptions::from_config`
    - [ ] Verify all fields are correctly mapped from Config
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): ContainerOptions field mapping`
- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
