# Plan: Tests — Config and Settings Full Field Coverage

## Phase 1: Config Field Tests

- [ ] Task: Add fixture or inline JSON for each missing field
    - [ ] `mounts`, `remoteEnv`, `containerEnv`, `shutdownAction`, `overrideCommand`,
          `forwardPorts`, `remoteUser`, `containerUser`, `runArgs`, `selinuxRelabel`
    - [ ] One test per field: present → correct value, absent → expected default
    - [ ] `cargo test` must pass
- [ ] Task: Test all six lifecycle hooks in string and array form
    - [ ] `initializeCommand`, `onCreateCommand`, `updateContentCommand`,
          `postCreateCommand`, `postStartCommand`, `postAttachCommand`
    - [ ] Both `"string"` and `["array", "form"]`
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(config): comprehensive Config field deserialization coverage`

## Phase 2: Settings Tests

- [ ] Task: Test all Settings fields
    - [ ] Engine: `docker`, `docker-compose`, `podman`, `podman-compose` all parse correctly
    - [ ] Unknown engine value → error
    - [ ] Default settings (missing file) → correct defaults
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(settings): full field and error path coverage`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate and coverage check
    - [ ] `cargo test`
    - [ ] `cargo llvm-cov --lib` — verify `config.rs` ≥85%, `settings.rs` ≥80%
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
