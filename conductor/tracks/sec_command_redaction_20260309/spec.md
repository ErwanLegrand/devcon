# Spec: Security — Redact Sensitive Values From Logged Commands

## Problem

`print_command` in `src/provider/mod.rs` prints the full command line to stdout before execution.
This includes `--env KEY=VALUE` arguments where VALUE may contain secrets (API keys, tokens,
passwords) sourced from `remoteEnv`. These values appear in terminal output, shell history of
CI log artifacts, and any log aggregation system that captures stdout.

## Goal

Redact the values of `--env KEY=VALUE` arguments in command output while keeping the key visible
for debuggability.

## Functional Requirements

- FR-001: In `print_command`, detect `--env` / `-e` arguments followed by `KEY=VALUE` or
  combined `--env=KEY=VALUE` forms.
- FR-002: Replace the VALUE portion with `***` in the printed output:
  `--env API_KEY=***` instead of `--env API_KEY=supersecret`.
- FR-003: The actual command passed to the subprocess is **not** modified — only the displayed
  string is redacted.
- FR-004: Keys with no value (env passthrough: `--env KEY`) are printed as-is.
- FR-005: Unit tests: `--env FOO=bar` → `--env FOO=***`, `--env FOO` unchanged,
  `--env=FOO=bar` → `--env=FOO=***`.
- FR-006: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Redacting values inside lifecycle hook command arguments.
- Structured logging / log levels (future).
