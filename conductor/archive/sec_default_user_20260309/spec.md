# Spec: Security — Warn When Container Runs as Root With No remoteUser Configured

## Problem

When `devcontainer.json` specifies neither `remoteUser` nor a `USER` instruction in the
Dockerfile, the container defaults to running as root (UID 0). Combined with DooD (Docker socket
mounted), this is a full host escape vector. The user is given no warning.

STRIDE classification: **Elevation of Privilege** (High).

## Goal

Detect when the effective container user will be root and emit a prominent warning before starting
the container.

## Functional Requirements

- FR-001: After the container is created but before `start`/`attach`, inspect the container's
  effective user via `docker inspect --format={{.Config.User}} <name>` (or podman equivalent).
- FR-002: If the effective user is empty string or `"root"` or `"0"` and no `remoteUser` is
  configured in `devcontainer.json`, emit a warning to stderr:
  ```
  WARNING: Container will run as root. Consider setting `remoteUser` in devcontainer.json.
  ```
- FR-003: The warning does not block execution (non-fatal). Use a dedicated `warn!` macro or
  `eprintln!` for visibility.
- FR-004: If `remoteUser` is set, skip the check entirely.
- FR-005: Add a `--no-root-check` flag to suppress the warning for intentional root containers.
- FR-006: Unit test: verify the warning is emitted for root user, suppressed for non-root, and
  suppressed with `--no-root-check`.

## Out of Scope

- Blocking root containers outright.
- Checking `containerUser` (separate field, lower priority).
