# Plan: Tests — PodmanCompose Provider Unit Coverage

## Phase 1: Arg Construction Tests

- [ ] Task: Test `PodmanCompose::create()` argument construction
    - [ ] Verify name, image, workspace, env vars appear in args
    - [ ] Verify mounts appear in args (after `fix_podman_mounts_parity` is done)
    - [ ] `cargo test` must pass
- [ ] Task: Test `PodmanCompose::exec()` and `exec_raw()` argument construction
    - [ ] Verify service name and command appear in correct positions
    - [ ] `cargo test` must pass
- [ ] Task: Test `PodmanCompose::attach()` shell selection
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): PodmanCompose arg construction coverage`

## Phase 2: Output Parsing Tests

- [ ] Task: Test `PodmanCompose::exists()` output parsing
    - [ ] Valid compose ps output with running container → true
    - [ ] Empty output → false
    - [ ] `cargo test` must pass
- [ ] Task: Test compose override YAML generation
    - [ ] SSH socket entry present when SSH_AUTH_SOCK is set
    - [ ] Remote env vars appear in service env section
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): PodmanCompose output parsing and override template`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate and coverage check
    - [ ] `cargo test`
    - [ ] `cargo llvm-cov --lib` — verify `podman_compose.rs` coverage ≥70%
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
