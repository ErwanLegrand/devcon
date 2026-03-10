# Spec: Feature — Configurable Timeout for Lifecycle Hooks

## Problem

Lifecycle hooks (all six: `initializeCommand`, `onCreateCommand`, `updateContentCommand`,
`postCreateCommand`, `postStartCommand`, `postAttachCommand`) run with no timeout. A hung hook
(e.g., an `npm install` that stalls, a waiting interactive prompt) will block `devcont` forever
with no escape mechanism except `Ctrl+C`.

## Goal

Apply a configurable timeout to each lifecycle hook execution. If the hook exceeds the timeout,
kill the process and return an error.

## Functional Requirements

- FR-001: Add optional `"hookTimeoutSeconds": <u32>` field to `devcontainer.json` (parsed into
  `Config`). Default: no timeout (current behaviour preserved).
- FR-002: When `hookTimeoutSeconds` is set, wrap host hook execution in a `std::thread` with a
  join timeout; send SIGTERM then SIGKILL if exceeded.
- FR-003: For in-container hooks, send `docker exec` / `podman exec` a `--timeout` flag if
  the runtime supports it, or use the same thread timeout strategy.
- FR-004: Timeout error message includes the hook name and configured timeout:
  `"postCreateCommand timed out after 120s"`.
- FR-005: A `--hook-timeout <seconds>` CLI flag overrides the config value for the current run.
- FR-006: Unit tests: hook completing within timeout → success; hook not completing → timeout error.

## Out of Scope

- Per-hook timeout configuration (same timeout applies to all hooks, simplification).
- Graceful shutdown sequence for in-container processes (future).
