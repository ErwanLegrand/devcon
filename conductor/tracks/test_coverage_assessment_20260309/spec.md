# Spec: Test Coverage Assessment

## Objective

Measure the current test coverage of `devcont`, identify which code paths are untested or
under-tested, and produce a prioritised plan for closing the most important coverage gaps.
This is an analytical track — it produces a report and new track stubs, not code changes.

## Scope

### Coverage Measurement

Run `cargo llvm-cov --summary-only` and `cargo llvm-cov --html` to get per-file and per-line
coverage data. Capture:
- Overall line coverage %
- Per-file line coverage %
- Specific uncovered lines and branches

### Gap Analysis

For each under-covered file (< 80% line coverage), identify:
1. Which functions/branches are not covered
2. Why they are not covered (no unit test, integration test only, dead code, etc.)
3. Risk level: what could go wrong if this path has a bug?
4. Test feasibility: can this path be unit-tested without a running container?

### Special Focus Areas

- `src/devcontainers/mod.rs` — lifecycle hook orchestration, `run()` happy/error paths
- `src/devcontainers/config.rs` — config parsing edge cases (missing fields, wrong types)
- `src/provider/utils.rs` — compose override template rendering
- Error paths in all providers (what happens when a command returns non-zero?)
- The `should_shutdown` / `ShutdownAction` logic in `Config`

### Coverage Baseline

Document the baseline at the time of assessment so improvements can be measured.

## Deliverable

A markdown report saved to `conductor/tracks/test_coverage_assessment_20260309/report.md`:

1. **Baseline coverage table** — per-file line coverage %, flagging files below 80%.
2. **Gap inventory** — for each gap: file, function/line range, gap type, risk level,
   feasibility of unit test vs integration test.
3. **Priority matrix** — gaps ranked by (risk × feasibility), highest priority first.
4. **Recommended tests** — for each high-priority gap, a sketch of the test case
   (what to set up, what to assert).
5. **Follow-up track stubs** — suggested conductor tracks to implement the highest-priority
   test additions.

## Output Artefact

`conductor/tracks/test_coverage_assessment_20260309/report.md`
