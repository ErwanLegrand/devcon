# Plan: Fix — Settings Load Silently Falls Back on Any Error

## Phase 1: Fix Settings::load()

- [ ] Task: Refactor `Settings::load()` error handling
    - [ ] Read `src/settings.rs` (or wherever `Settings::load` lives)
    - [ ] Separate "file not found" → `Ok(default)` from other errors → `Err`
    - [ ] Use `io::ErrorKind::NotFound` to distinguish missing file
    - [ ] Return `Err` with path context for parse errors and IO errors
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(settings): error on malformed settings file, not silent fallback`

## Phase 2: Tests

- [ ] Task: Unit tests for `Settings::load()` error paths
    - [ ] Non-existent settings file → `Ok(Settings::default())`
    - [ ] Valid settings file → `Ok(Settings { ... })` with expected values
    - [ ] Existing file with invalid content → `Err` containing path
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(settings): error handling for missing vs invalid settings file`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
