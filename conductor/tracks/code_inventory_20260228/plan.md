# Plan: Code Inventory

## Phase 1: Audit and Findings Document

- [x] Task: Run static analysis
    - [x] Run `cargo clippy -W dead-code -W unreachable-code -- -D warnings` and
          record all new warnings
    - [x] List every `#[allow(dead_code)]` annotation in `src/` (grep output)
    - [x] Confirm `src/provider/utils.rs` is empty
    - [x] Check `DockerCompose` and `PodmanCompose` struct fields: run
          `cargo clippy` without their struct-level `#[allow(dead_code)]` to see
          which fields trigger warnings
- [x] Task: Write `findings.md`
    - [x] Create `conductor/tracks/code_inventory_20260228/findings.md`
    - [x] Document all 7 pre-identified items (Items 1–7 from spec) with
          location, description, and proposed decision
    - [x] Add any additional items discovered during static analysis
- [x] Task: Conductor - User Manual Verification 'Phase 1: Audit and Findings Document' (Protocol in workflow.md)

## Phase 2: Implement Removals

- [x] Task: Remove `Error::Provider` (Item 1)
    - [x] Delete the `Provider(String)` variant and its `#[allow(dead_code)]` from
          `src/error.rs`
    - [x] Remove the `SettingsLoad` variant too if it is also unused
          (verify first) — SettingsLoad IS used in settings.rs:61, retained
    - [x] `cargo test` must pass
    - [x] Commit: `refactor(error): remove unused Provider error variant`
- [x] Task: Remove or wire `Config::file` (Item 2)
    - [x] If decision is Remove: delete the field and its `#[allow(dead_code)]`
          from `src/devcontainers/config.rs`; update `Config::parse` if needed
    - [x] `cargo test` must pass
    - [x] Commit: `refactor(config): remove/retain Config::file per findings`
- [x] Task: Remove or wire `Build::context` (Item 3)
    - [x] If decision is Retain: replace `#[allow(dead_code)]` with spec citation comment
    - [x] `cargo test` must pass
    - [x] Commit: `refactor(config): replace allow(dead_code) with spec comment for Build::context`
- [x] Task: Address `ShutdownAction::StopCompose` (Item 4)
    - [x] If decision is Retain: add inline comment explaining why StopCompose ==
          StopContainer is currently acceptable
    - [x] `cargo test` must pass
    - [x] Commit: combined with Build::context commit
- [x] Task: Delete `src/provider/utils.rs` (Item 5)
    - [x] Delete `src/provider/utils.rs`
    - [x] Remove `pub mod utils;` from `src/provider/mod.rs` if it exists — not present
    - [x] `cargo build` must pass
    - [x] Commit: `chore: delete empty provider/utils.rs`
- [x] Task: Conductor - User Manual Verification 'Phase 2: Implement Removals (partial)' (Protocol in workflow.md)

## Phase 3: Implement Refactors

- [x] Task: Clean up unused compose struct fields (Item 6)
    - [x] Remove `#[allow(dead_code)]` from `DockerCompose` struct
    - [x] Remove `#[allow(dead_code)]` from `PodmanCompose` struct
    - [x] Remove `directory`, `forward_ports`, `run_args` from both structs
    - [x] Update `build_provider` in `src/devcontainers/mod.rs`
    - [x] Update struct initializers in `tests/integration.rs`
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `refactor(provider): remove unused compose struct fields`
- [x] Task: Consolidate `create_docker_compose()` duplication (Item 7)
    - [x] Extract shared logic into `create_compose_override(service: &str)` in
          `src/provider/utils.rs`
    - [x] Replace both `DockerCompose::create_docker_compose()` and
          `PodmanCompose::create_docker_compose()` with calls to the shared function
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `refactor(provider): extract shared create_compose_override helper`
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Implement Refactors' (Protocol in workflow.md)

## Phase 4: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test` (unit tests) ✓
    - [ ] `cargo test --test integration` ✓
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓ (no suppressed items
          without findings.md justification)
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
    - [ ] Verify no `#[allow(dead_code)]` remains without a findings justification
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Quality Gate' (Protocol in workflow.md)
