# Spec: Tests — Devcontainer Lifecycle run(), rebuild(), All Hook Paths

## Problem

`src/devcontainers/mod.rs` has 0% unit test coverage for `run()` and `rebuild()`. These are the
most critical functions in the codebase — they orchestrate the full container lifecycle including
all six hook types. Any regression here is invisible to the test suite.

Baseline coverage: `src/devcontainers/mod.rs` is at ~30% (only `exec_hook` and helpers tested).

## Goal

Add unit tests for `run()` and `rebuild()` using MockProvider, covering the happy path and all
hook failure modes. Target: ≥80% line coverage for `mod.rs`.

## Functional Requirements

- FR-001: Tests use the existing `MockProvider` (already in `#[cfg(test)]`).
- FR-002: Happy path test: `run()` with a config that has all six hooks set, all hooks succeed.
  Verify MockProvider received all expected calls in order.
- FR-003: Hook failure tests: each hook returning `Ok(false)` (or `Err`) causes `run()` to
  return `Err` with the hook name in the message.
- FR-004: `rebuild()` happy path test.
- FR-005: `Config` with no hooks: `run()` succeeds without calling exec.
- FR-006: Image-based config (no dockerfile): build is skipped.
- FR-007: Tests use `Config` fixtures from `tests/fixtures/` or inline JSON strings.
- FR-008: `cargo test` must pass and coverage must reach ≥80% for `mod.rs`.

## Out of Scope

- Integration tests (separate track).
- Testing provider implementations (separate coverage tracks).
