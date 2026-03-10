# Plan: Refactor — Tighten Provider Module Visibility

## Phase 1: Audit

- [ ] Task: Enumerate all `pub` items in `src/provider/` and `src/devcontainers/`
    - [ ] List: function name, current visibility, actual usage scope

## Phase 2: Apply Visibility Changes

- [ ] Task: Change `print_command` from `pub` to `pub(crate)` in `src/provider/mod.rs`
    - [ ] `cargo build` must pass
- [ ] Task: Tighten visibility of `extract_container_id`, `running_args`, `rm_args` in provider files
    - [ ] Change to `fn` (private) if only used in same file
    - [ ] Change to `pub(crate)` if used across provider modules
    - [ ] `cargo build` must pass
- [ ] Task: Tighten `BuildSource` variants and any other public enums/structs
    - [ ] `cargo build` must pass
- [ ] Task: Tighten visibility in `src/devcontainers/`
    - [ ] `cargo build` must pass
    - [ ] Commit: `refactor(visibility): tighten pub to pub(crate) or private throughout`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings` (no dead_code warnings)
    - [ ] `cargo fmt --check`
