# Spec: Security — Prevent Path Traversal in Workspace and Dockerfile Paths

## Problem

Several user-controlled paths from `devcontainer.json` are used without validation:

- `workspaceFolder` / `workspaceMount` — mounted into the container as-is.
- `build.context` and `build.dockerfile` — passed to `docker build` without normalisation.
- `mounts[].source` — host-side mount paths appended to docker/podman run.

A `devcontainer.json` with `"build": {"dockerfile": "../../../../etc/passwd"}` or
`"workspaceFolder": "/root"` could expose host filesystem contents or cause unintended builds.

STRIDE classification: **Tampering / Information Disclosure** (Critical).

## Goal

Canonicalise and validate all user-supplied paths before use, rejecting paths that escape the
workspace root or resolve to sensitive system locations.

## Functional Requirements

- FR-001: All paths from `devcontainer.json` must be resolved relative to the workspace root
  using `Path::canonicalize` or a safe equivalent that does not require the path to exist.
- FR-002: `build.dockerfile` must resolve to a path within the workspace root; abort if it
  escapes (e.g., contains `..` components that leave the workspace).
- FR-003: `build.context` must resolve to a path within the workspace root or an explicit
  absolute path the user has configured (warn on absolute paths outside workspace).
- FR-004: `mounts[].source` paths that are absolute are allowed (user intent), but relative
  paths must be validated to stay within the workspace.
- FR-005: A helper `fn validate_within_root(root: &Path, candidate: &Path) -> Result<PathBuf>`
  is introduced in `src/devcontainers/paths.rs` (new file).
- FR-006: Unit tests: path traversal attempts (`../../etc`, symlinks pointing out, absolute
  paths outside workspace) all return an error.

## Out of Scope

- Restricting which absolute host paths can be mounted (this is `sec_runargs_allowlist`).
- Validating paths that only exist at runtime inside the container.
