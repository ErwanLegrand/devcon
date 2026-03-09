# Spec: Fix — Dockerfile Path Resolution Asymmetry Between Docker and Podman

## Problem

`Docker::build()` constructs the dockerfile path as
`workspace_folder.join(&config.build.dockerfile)` while `Podman::build()` uses a different
resolution strategy. When `build.dockerfile` is a relative path the two providers resolve it
against different roots, producing different behaviour for the same `devcontainer.json`.

## Goal

Unify dockerfile path resolution across all four providers so that relative paths in
`build.dockerfile` are consistently resolved relative to `build.context` (or the workspace root
if context is unset), matching the Dev Containers spec.

## Functional Requirements

- FR-001: Extract `fn resolve_dockerfile_path(workspace: &Path, build: &Build) -> PathBuf`
  into a shared location (`src/devcontainers/paths.rs` or `src/provider/utils.rs`).
- FR-002: The helper resolves `build.dockerfile` relative to `build.context` if context is
  set, otherwise relative to the workspace root.
- FR-003: All four providers use this helper for dockerfile path resolution.
- FR-004: Unit tests: relative dockerfile with explicit context, relative dockerfile without
  context, absolute dockerfile path.
- FR-005: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Validating that the resolved path exists (done at build time by the runtime).
- Path traversal checks (tracked in `sec_path_traversal`).
