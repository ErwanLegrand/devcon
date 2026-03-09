# Spec: Refactor — Extract ContainerOptions Struct to Decouple Devcontainer From Provider Args

## Problem

`Devcontainer::run()` assembles container arguments (name, image, mounts, env vars, run args,
workspace folder, user) inline before calling provider methods. As more `devcontainer.json` fields
are added, this function grows unbounded. Provider methods also take several positional parameters
that could be a struct.

## Goal

Extract a `ContainerOptions` (or `CreateOptions`) struct that carries all container creation
parameters, built once in `run()` / `rebuild()` and passed to the provider.

## Functional Requirements

- FR-001: Define `pub(crate) struct ContainerOptions` in `src/devcontainers/options.rs`
  (new file) with fields: `name`, `image`, `workspace_folder`, `mounts`, `remote_env`,
  `container_env`, `run_args`, `remote_user`, `override_command`.
- FR-002: `Devcontainer::run()` builds `ContainerOptions` from `Config` once, then passes it
  to provider methods.
- FR-003: Provider `create()` (and other methods that need it) accept `&ContainerOptions`
  instead of individual positional arguments.
- FR-004: All four providers updated to use `ContainerOptions`.
- FR-005: Existing tests pass; no behavioral change.
- FR-006: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Adding new fields to `ContainerOptions` beyond what exists today.
- Changing lifecycle hook signatures.
