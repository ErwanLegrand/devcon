# Plan: Fix — Propagate Non-Zero Exit From In-Container Lifecycle Hooks

## Phase 1: Fix exec_hook Return Value Handling

- [ ] Task: Audit all `exec_hook` call sites in `src/devcontainers/mod.rs`
    - [ ] List every call to `exec_hook` in `run()` and `rebuild()`
    - [ ] Identify which ones currently ignore `Ok(false)`
- [ ] Task: Abort on hook failure in `run()`
    - [ ] For `postCreateCommand`, `postStartCommand`, `postAttachCommand`: if `exec_hook`
          returns `Ok(false)`, return `Err(io::Error::new(Other, "<hookName> failed"))`
    - [ ] `cargo build` must pass
- [ ] Task: Abort on hook failure in `rebuild()` (if applicable)
    - [ ] Apply the same pattern for any hooks called in `rebuild()`
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(devcontainer): abort lifecycle on non-zero exit from in-container hooks`

## Phase 2: Tests

- [ ] Task: Unit tests using MockProvider
    - [ ] MockProvider::exec returns `Ok(false)` → `run()` returns Err naming the hook
    - [ ] MockProvider::exec returns `Ok(true)` → lifecycle continues
    - [ ] Correct hook name appears in the error message
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): in-container hook exit propagation`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
