# Spec: Feature — Full build.context and build.args Support for All Providers

## Problem

`build.context` and `build.args` from `devcontainer.json` are parsed into `Config::Build` but
not all providers correctly pass them to the build command:

- `build.context` is sometimes ignored, defaulting to the workspace root.
- `build.args` (`{"ARG": "value"}`) are not passed as `--build-arg ARG=value` flags.

## Goal

All four providers correctly use `build.context` when set, and pass all `build.args` as
`--build-arg` flags to the build command.

## Functional Requirements

- FR-001: `Docker::build()`: use `config.build.context` as the build context path if set,
  workspace root otherwise.
- FR-002: `Docker::build()`: append `--build-arg KEY=VALUE` for each entry in `build.args`.
- FR-003: `Podman::build()`: apply the same changes as Docker.
- FR-004: `DockerCompose::build()` and `PodmanCompose::build()`: inject `build.args` into
  the compose override's `build.args` map.
- FR-005: Unit tests: args are passed in correct format, context path is used when present.
- FR-006: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Validating build arg key format (tracked in `sec_input_sanitisation`).
- Validating context path (tracked in `sec_path_traversal`).
