# Spec: Fix — Settings Load Silently Falls Back on Any Error

## Problem

`Settings::load()` returns a default `Settings` on any error (file not found, parse error,
permission denied). A misconfigured or corrupted settings file is silently ignored, leading to
confusing behaviour where user-specified engine preferences have no effect.

## Goal

Distinguish between "settings file does not exist" (acceptable — use defaults) and "settings
file exists but cannot be parsed" (should be an error).

## Functional Requirements

- FR-001: If the settings file does not exist, return `Ok(Settings::default())`.
- FR-002: If the settings file exists but cannot be read (permission denied, IO error), return
  an error with a clear message identifying the file path.
- FR-003: If the settings file exists but cannot be parsed (invalid TOML / schema mismatch),
  return an error with a clear message showing the parse error and file path.
- FR-004: Emit a one-line notice to stderr when falling back to defaults (file not found case
  only) if a debug/verbose flag is set.
- FR-005: Unit tests: missing file → Ok(default), existing valid file → Ok(settings),
  existing invalid file → Err with path in message.

## Out of Scope

- Migrating old settings formats.
- Interactive settings editing.
