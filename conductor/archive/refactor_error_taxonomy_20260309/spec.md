# Spec: Refactor — Introduce Typed Error Enum Instead of io::Error for All Failures

## Problem

All failure paths in `devcont` use `std::io::Error::new(ErrorKind::Other, "message")`, which:
- Loses structured error information (no machine-readable error code).
- Makes it impossible for callers or tests to match on specific failure kinds.
- Produces poor error messages when errors propagate through multiple layers.

The `Error` type in `src/lib.rs` defines variants but they are not consistently used — many
call sites bypass it with raw `io::Error`.

## Goal

Consolidate all error paths to use the existing `Error` enum (or an expanded version), ensuring
every failure variant is named and matchable.

## Functional Requirements

- FR-001: Audit every `io::Error::new(Other, ...)` call site and replace with the appropriate
  `Error` variant or a new variant if needed.
- FR-002: New variants may include: `HookFailed { hook: String }`, `InvalidConfig(String)`,
  `ProviderError { provider: String, message: String }`, `PathTraversal(PathBuf)`.
- FR-003: All public functions returning `io::Result` are updated to return `Result<T, Error>`
  (or keep `io::Result` with a proper `From<Error> for io::Error` impl).
- FR-004: Error messages use structured display: `"postCreateCommand failed (exit 1)"` not
  just `"hook failed"`.
- FR-005: Existing tests updated; new tests verify error variant matching.
- FR-006: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Switching to `anyhow` or `thiserror` (evaluate in this track; adopt if beneficial).
- Error reporting to users (UI layer, future).
