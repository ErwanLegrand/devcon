# Spec: Security — Sanitise Container Names and Label Values Used in Shell Commands

## Problem

`safe_name()` converts the workspace name to a container name but the output is used directly
in shell-interpolated filter strings (e.g., `--filter name=<safe_name>`). If `safe_name()`
allows characters that have special meaning in shell or in Docker filter expressions (e.g., `$`,
`*`, `(`, `)`, `\`), they could cause unintended command behavior.

Similarly, environment variable keys/values from `remoteEnv` are passed to `--env` flags without
validation of the key format (keys must match `[A-Za-z_][A-Za-z0-9_]*`).

## Goal

Validate that container names and env var keys conform to safe character sets before use.

## Functional Requirements

- FR-001: `safe_name()` output must match `^[a-zA-Z0-9][a-zA-Z0-9_.-]*$`. Return an error at
  startup if the workspace directory name cannot be made safe.
- FR-002: `remoteEnv` keys must match `^[A-Za-z_][A-Za-z0-9_]*$`. Reject invalid keys with a
  clear error naming the offending key.
- FR-003: `remoteEnv` values must not contain null bytes. Reject with clear error.
- FR-004: Unit tests for all three validation rules with valid and invalid inputs.
- FR-005: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Validating `containerEnv` (same pattern, add in same PR).
- Validating hook command strings (shell is the runtime validator there).
