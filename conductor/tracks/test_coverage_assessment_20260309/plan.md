# Plan: Test Coverage Assessment

## Phase 1: Measure Baseline

- [ ] Task: Run coverage tooling and capture baseline
    - [ ] Run `cargo llvm-cov --summary-only` (or `cargo tarpaulin` if llvm-cov unavailable)
    - [ ] Capture per-file line coverage percentages
    - [ ] Identify all files below 80% threshold
    - [ ] Note which lines/branches are uncovered (from llvm-cov HTML or lcov output)

## Phase 2: Gap Analysis

- [ ] Task: Analyse gaps in devcontainers/ modules
    - [ ] `src/devcontainers/mod.rs` — `run()`, `post_create()`, `create()`, hook execution paths
    - [ ] `src/devcontainers/config.rs` — edge cases: missing fields, malformed values, `ShutdownAction`
    - [ ] `src/devcontainers/one_or_many.rs` — already well-tested; verify

- [ ] Task: Analyse gaps in provider/ modules
    - [ ] Per-provider: which methods have no unit tests?
    - [ ] `src/provider/utils.rs` — template rendering, SSH socket absent case
    - [ ] Error paths: what happens when commands fail (non-zero exit)?

- [ ] Task: Classify and prioritise gaps
    - [ ] For each gap: assign risk (what breaks if bugged?) and feasibility (unit vs integration test needed)
    - [ ] Build priority matrix: risk × feasibility

## Phase 3: Report

- [ ] Task: Write the full report to `conductor/tracks/test_coverage_assessment_20260309/report.md`
    - [ ] Baseline coverage table (per-file %)
    - [ ] Gap inventory with risk and feasibility ratings
    - [ ] Priority matrix
    - [ ] Recommended test sketches for high-priority gaps
    - [ ] Follow-up track stubs
    - [ ] Commit: `docs(conductor): add test coverage assessment report`

- [ ] Task: Conductor - User Manual Verification 'Test Coverage Report' (Protocol in workflow.md)
