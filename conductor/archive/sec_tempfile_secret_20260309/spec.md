# Spec: Security — Fix Compose Override Temp File Permissions and Cleanup

## Problem

`create_compose_override` in `src/provider/utils.rs` writes a Docker Compose override YAML file
to `env::temp_dir()`. Two issues:

1. **File permissions**: The file is created with default `umask`-derived permissions (typically
   0o644 on Linux), making it world-readable. The override file contains environment variable
   values and SSH socket paths from the user's session — sensitive data.
2. **No cleanup on failure**: The override file is never deleted if the container creation step
   fails mid-way. Orphaned files accumulate and contain session data.

STRIDE classification: **Information Disclosure** (Critical).

## Goal

Ensure the compose override file is created with restrictive permissions (0o600) and is reliably
deleted when no longer needed, whether creation succeeds or fails.

## Functional Requirements

- FR-001: Create the temp file with mode 0o600 (owner read/write only). Use `OpenOptions` with
  explicit `mode(0o600)` before writing.
- FR-002: Return a guard/handle from `create_compose_override` that deletes the file when dropped
  (`Drop` impl or explicit cleanup call from callers).
- FR-003: All callers of `create_compose_override` in Docker Compose and Podman Compose providers
  must ensure the file is cleaned up after use (on both success and error paths).
- FR-004: Unit test: verify the created file has mode 0o600.
- FR-005: Unit test: verify file is removed after the guard is dropped.
- FR-006: If atomic temp-file creation is needed, use `tempfile` crate's `NamedTempFile` with
  explicit `persist` only if needed; otherwise manual `O_CREAT | O_EXCL` with mode 0o600.

## Out of Scope

- Encrypting the compose override file content.
- Changing the compose override template itself.
