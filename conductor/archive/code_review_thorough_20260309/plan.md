# Plan: Thorough Code Review

## Phase 1: Per-File Review

- [x] Task: Review core orchestration — `src/devcontainers/mod.rs`, `config.rs`, `one_or_many.rs`
    - [x] Lifecycle hook exit codes silently ignored in post_create and run (High)
    - [x] initializeCommand failure handling reviewed
    - [x] OneOrMany serde edge cases checked
    - [x] Recorded findings with file, line range, severity, category

- [x] Task: Review provider layer — `src/provider/mod.rs`, `docker.rs`, `podman.rs`
    - [x] exists()/running() use prefix filter not exact match — false positives (High)
    - [x] Result<bool> on Provider trait allows silent ignored exits (Medium)
    - [x] All struct fields pub when pub(crate) would suffice (Medium)
    - [x] Podman lacks mounts field entirely — parity gap with Docker (Medium)
    - [x] Recorded findings

- [x] Task: Review compose providers — `src/provider/docker_compose.rs`, `podman_compose.rs`, `utils.rs`
    - [x] Compose override written to fixed world-readable temp path — concurrent projects overwrite each other (High)
    - [x] DockerCompose::exists omits override file arg, inconsistent (High)
    - [x] SSH socket path in YAML without sanitisation (Medium)
    - [x] Recorded findings

- [x] Task: Review settings, error handling, and entry points — `src/settings.rs`, `src/error.rs`, `src/main.rs`
    - [x] Settings fields pub when pub(crate) suffices (Medium)
    - [x] Error type coarse — std::io::Error used everywhere (Medium)
    - [x] Recorded findings

- [x] Task: Review tests — `tests/integration.rs`
    - [x] Test isolation and fixture completeness reviewed
    - [x] Recorded findings

## Phase 2: Report

- [x] Task: Write the full report to `conductor/tracks/code_review_thorough_20260309/report.md`
    - [x] 42 findings across 13 files (7 High, ~15 Medium, remainder Low/Info)
    - [x] Summary table by severity and category
    - [x] 10 follow-up track stubs
    - [x] Commit: `docs(conductor): add thorough code review report` [bf8a731]

- [ ] Task: Conductor - User Manual Verification 'Code Review Report' (Protocol in workflow.md)
