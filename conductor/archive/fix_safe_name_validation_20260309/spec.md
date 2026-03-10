# Spec: Fix — safe_name() Silently Truncates Unicode — Validate Output

## Problem

`safe_name()` strips non-ASCII characters from the workspace directory name to produce a
Docker-compatible container name. For workspace paths composed entirely of Unicode characters
(e.g., Chinese or Arabic directory names), this produces an empty string, which is then used
as the container name, causing a Docker error that is difficult to diagnose.

## Goal

Return an error early if `safe_name()` produces an empty or invalid result, with a clear
message directing the user to rename their workspace directory.

## Functional Requirements

- FR-001: After stripping unsafe characters, check that the result is non-empty.
- FR-002: If the result is empty, return `Err` with a message:
  `"Cannot derive a container name from workspace path '<path>'. Rename the directory to use ASCII characters."`.
- FR-003: If the result starts with a non-alphanumeric character (Docker requires names to
  start with a letter or digit), prepend `"dev-"` as a safe prefix and log a notice.
- FR-004: Unit tests: all-unicode path → error, mixed path → prefix stripped, valid path →
  unchanged, path starting with `-` → prepend `"dev-"`.

## Out of Scope

- Supporting Unicode container names (Docker doesn't allow them).
- Configuring a custom container name override (future: `"containerName"` field in config).
