# Plan: PodmanCompose Unit Tests — running(), cp(), rm() Bug Fix Coverage

## Phase 1: Unit Tests

- [x] Task: Unit test for `cp()` ID extraction logic
    - [x] Extract the first-non-empty-line logic from `cp()` into a private `extract_container_id(output: &str) -> &str` helper
    - [x] Add unit tests in `src/provider/podman_compose.rs` covering:
        - single-line output → returns that line trimmed
        - multi-line output with blank leading lines → returns first non-empty line
        - empty string → returns ""
    - [x] `cargo test` must pass
    - [x] Commit: `test(provider): unit test PodmanCompose cp() container ID extraction`
- [x] Task: Unit tests for `running()` and `rm()` command args
    - [x] Extract command-arg construction for `running()` and `rm()` into private helpers returning `Vec<String>` (testable without exec)
    - [x] Add unit tests verifying:
        - `running()` args include `--filter status=running` and do NOT include `--format`
        - `rm()` args do NOT include `--rmi` or `all`
    - [x] `cargo test` must pass
    - [x] Commit: `test(provider): unit test PodmanCompose running() and rm() command args`

## Phase 2: Quality Gate

- [x] Task: Run full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
