# Spec: Tests — exec_host_hook, OneOrMany, safe_name, and Utility Functions

## Problem

Several utility functions in `src/devcontainers/` have low or no test coverage:

- `exec_host_hook`: 0% coverage (host-side hook execution, critical path).
- `OneOrMany` deserialization edge cases: missing coverage for `null`, integers, nested arrays.
- `safe_name()`: no tests for Unicode stripping, empty result, edge cases.
- `Config::parse` error paths: only the happy path is tested via fixtures.

## Goal

Add targeted unit tests for each of these functions. Target: ≥80% line coverage for each.

## Functional Requirements

- FR-001: `exec_host_hook` tests (using real process execution with short-lived commands):
    - Command that exits 0 → returns `Ok(true)`.
    - Command that exits 1 → returns `Ok(false)`.
    - Non-existent command → returns `Err`.
    - Many form with space in arg → arg preserved without splitting.
- FR-002: `OneOrMany` deserialization edge cases (extend existing tests):
    - Deserialize `null` → Err (or Ok with empty, document expected behaviour).
    - Deserialize integer → Err.
    - Deserialize nested array `[["a"]]` → Err.
    - Empty string `""` → Ok(One("")).
- FR-003: `safe_name()` tests:
    - All-ASCII name → unchanged (minus Docker-invalid chars).
    - Name with spaces → spaces replaced.
    - All-Unicode name → error (after `fix_safe_name_validation` track).
    - Name starting with digit → valid.
- FR-004: `Config::parse` error paths:
    - Missing required field → Err.
    - Invalid JSON5 → Err.
    - Extra unknown field → Ok (serde default).
- FR-005: `cargo test` must pass.

## Out of Scope

- Integration tests requiring a running container runtime.
