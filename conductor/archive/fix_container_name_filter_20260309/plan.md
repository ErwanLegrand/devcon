# Plan: Fix — Container Name Filter in exists() and running() is Too Broad

## Phase 1: Fix Exact Name Matching

- [ ] Task: Fix `Docker::exists()` and `Docker::running()`
    - [ ] Use `--filter name=^/<name>$` or parse output and compare exact name
    - [ ] `cargo build` must pass
- [ ] Task: Fix `Podman::exists()` and `Podman::running()`
    - [ ] Apply same fix
    - [ ] `cargo build` must pass
- [ ] Task: Fix `DockerCompose::exists()` / `running()` and `PodmanCompose::exists()` / `running()`
    - [ ] Apply same fix where applicable
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(provider): use exact name match in exists() and running() filters`

## Phase 2: Tests

- [ ] Task: Unit tests for name filter logic
    - [ ] Filter for `"foo"` does not match container named `"foobar"`
    - [ ] Filter for `"foo"` does not match container named `"barfoo"`
    - [ ] Filter for `"foo"` matches container named `"foo"` exactly
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): exact container name filter coverage`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
