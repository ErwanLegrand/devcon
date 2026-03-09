# Spec: Feature — Structured Audit Log for All Container Lifecycle Events

## Problem

`devcont` currently prints raw command lines via `print_command`. There is no persistent record
of what commands were run, what hooks executed, or what the container name/image was. For
security-conscious users (and CI pipelines), an audit trail is valuable for post-incident review.

## Goal

Write a structured append-only audit log to `~/.local/share/devcont/audit.log` (XDG compliant)
capturing key lifecycle events.

## Functional Requirements

- FR-001: Log entries are newline-delimited JSON (`{"timestamp": "...", "event": "...", ...}`).
- FR-002: Events to log: `container_start`, `container_stop`, `container_rebuild`,
  `hook_executed` (with hook name, command, exit code), `command_run` (provider command, redacted).
- FR-003: Log file location: `$XDG_DATA_HOME/devcont/audit.log` or
  `~/.local/share/devcont/audit.log` as fallback.
- FR-004: Log file is created with mode 0o600.
- FR-005: If the log directory does not exist, create it (0o700).
- FR-006: If writing to the log fails (disk full, permission denied), emit a warning to stderr
  but do not abort the operation.
- FR-007: A `--no-audit-log` flag disables logging for a session.
- FR-008: Unit tests: log entry format, redaction of env values in `command_run` events,
  graceful handling of write failure.

## Out of Scope

- Log rotation (future).
- Remote log shipping.
- Reading or querying the audit log via CLI (future `devcont audit` subcommand).
