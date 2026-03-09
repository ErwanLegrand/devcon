# Plan: PodmanCompose Unit Tests — running(), cp(), rm() Bug Fix Coverage

## Phase 1: Unit Tests

- [ ] Task: Unit test for `cp()` ID extraction logic
    - [ ] Extract the first-non-empty-line logic from `cp()` into a private `extract_container_id(output: &str) -> &str` helper
    - [ ] Add unit tests in `src/provider/podman_compose.rs` covering:
        - single-line output → returns that line trimmed
        - multi-line output with blank leading lines → returns first non-empty line
        - empty string → returns ""
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): unit test PodmanCompose cp() container ID extraction`
- [ ] Task: Unit tests for `running()` and `rm()` command args
    - [ ] Extract command-arg construction for `running()` and `rm()` into private helpers returning `Vec<String>` (testable without exec)
    - [ ] Add unit tests verifying:
        - `running()` args include `--filter status=running` and do NOT include `--format`
        - `rm()` args do NOT include `--rmi` or `all`
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): unit test PodmanCompose running() and rm() command args`

## Phase 2: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
