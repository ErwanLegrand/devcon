# Plan: Thorough Code Review

## Phase 1: Per-File Review

- [ ] Task: Review core orchestration — `src/devcontainers/mod.rs`, `config.rs`, `one_or_many.rs`
    - [ ] Check lifecycle hook ordering and error semantics
    - [ ] Check config parsing completeness and validation gaps
    - [ ] Check `OneOrMany` serde edge cases
    - [ ] Record findings with file, line range, severity, category

- [ ] Task: Review provider layer — `src/provider/mod.rs`, `docker.rs`, `podman.rs`
    - [ ] Check Provider trait API design
    - [ ] Check Docker and Podman exec/create/build command construction
    - [ ] Check for unwrap/expect, clone overuse, pub visibility
    - [ ] Record findings

- [ ] Task: Review compose providers — `src/provider/docker_compose.rs`, `podman_compose.rs`, `utils.rs`
    - [ ] Check compose command construction and override file handling
    - [ ] Check temp file security (world-readable `/tmp`)
    - [ ] Check for duplicated logic vs shared utils
    - [ ] Record findings

- [ ] Task: Review settings, error handling, and entry points — `src/settings.rs`, `src/error.rs`, `src/main.rs`
    - [ ] Check settings loading and defaults
    - [ ] Check error type design and actionability
    - [ ] Check CLI argument handling
    - [ ] Record findings

- [ ] Task: Review tests — `tests/integration.rs`
    - [ ] Check for test isolation, hardcoded assumptions, missing assertions
    - [ ] Check fixture completeness
    - [ ] Record findings

## Phase 2: Report

- [ ] Task: Write the full report to `conductor/tracks/code_review_thorough_20260309/report.md`
    - [ ] Per-file findings (severity-rated, with line ranges)
    - [ ] Summary table (counts by severity and category)
    - [ ] Follow-up track stubs for clustered findings
    - [ ] Commit: `docs(conductor): add thorough code review report`

- [ ] Task: Conductor - User Manual Verification 'Code Review Report' (Protocol in workflow.md)
