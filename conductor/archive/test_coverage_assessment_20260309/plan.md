# Plan: Test Coverage Assessment

## Phase 1: Measure Baseline

- [x] Task: Run coverage tooling and capture baseline
    - [x] Ran `cargo llvm-cov --summary-only`
    - [x] Overall coverage: **54.32%** — below 80% target
    - [x] Per-file coverage captured; all files below 80% identified

## Phase 2: Gap Analysis

- [x] Task: Analyse gaps in devcontainers/ modules
    - [x] `src/devcontainers/mod.rs` — 18% line coverage; `run()`, `post_create()`, `create()`, `exec_host_hook` all lack unit tests; MockProvider exists but not wired to struct method tests
    - [x] `src/devcontainers/config.rs` — edge cases: `ShutdownAction::None`, missing fields, malformed values
    - [x] `src/devcontainers/one_or_many.rs` — well-tested; verified

- [x] Task: Analyse gaps in provider/ modules
    - [x] `PodmanCompose` — 37.81% coverage; most methods skipped in CI when podman-compose absent
    - [x] `Docker::build` Image-pull path — no integration test
    - [x] `src/provider/utils.rs` — `create_compose_override` without SSH_AUTH_SOCK not tested
    - [x] Error paths (non-zero exit) untested across providers

- [x] Task: Classify and prioritise gaps
    - [x] High: `Devcontainer::run()` unit tests, PodmanCompose coverage
    - [x] Medium: `exec_host_hook`, `rebuild`, `post_create`, image-pull path
    - [x] Low: ShutdownAction::None, create_compose_override no-SSH path, Settings, get_project_directory
    - [x] 8 follow-up track stubs defined

## Phase 3: Report

- [x] Task: Write the full report to `conductor/tracks/test_coverage_assessment_20260309/report.md`
    - [x] Baseline coverage table (per-file %)
    - [x] Gap inventory with risk and feasibility ratings
    - [x] Priority matrix
    - [x] Recommended test sketches for high-priority gaps
    - [x] 8 follow-up track stubs
    - [x] Commit: `docs(conductor): add test coverage assessment report` [8716469]

- [ ] Task: Conductor - User Manual Verification 'Test Coverage Report' (Protocol in workflow.md)
