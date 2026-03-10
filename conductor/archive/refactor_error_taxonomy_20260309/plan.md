# Plan: Refactor — Introduce Typed Error Enum

## Phase 1: Audit and Expand Error Enum

- [ ] Task: Audit all `io::Error::new(Other, ...)` call sites
    - [ ] List every occurrence in `src/` with file and line number
    - [ ] Categorise into: hook failures, config errors, provider errors, path errors
- [ ] Task: Expand `Error` enum in `src/lib.rs`
    - [ ] Add variants: `HookFailed { hook: String, exit_code: Option<i32> }`,
          `InvalidConfig(String)`, `ProviderError { provider: String, message: String }`,
          `PathError(String)`
    - [ ] Implement `Display` for all new variants
    - [ ] Ensure `From<Error> for io::Error` covers new variants
    - [ ] `cargo build` must pass

## Phase 2: Replace Raw io::Error Calls

- [ ] Task: Update `src/devcontainers/mod.rs` to use typed errors
    - [ ] Replace all `io::Error::new(Other, ...)` with appropriate `Error` variants
    - [ ] `cargo build` must pass
- [ ] Task: Update provider files to use typed errors
    - [ ] Apply same replacements in all four provider files
    - [ ] `cargo build` must pass
    - [ ] Commit: `refactor(error): replace raw io::Error with typed Error variants`

## Phase 3: Tests and Quality Gate

- [ ] Task: Update and add tests for error variant matching
    - [ ] Tests that match on `Error::HookFailed` etc. (not just `is_err()`)
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(error): typed error variant matching`
- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
