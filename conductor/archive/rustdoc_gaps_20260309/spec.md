# Spec: Docs — Fill Rustdoc Gaps on Public and Complex Functions

## Problem

The documentation assessment identified 12 rustdoc gaps. The most impactful missing docs are on
public functions and modules that form the library surface. Low-priority items (private struct
fields) are included for completeness.

## Rustdoc Gap Table

| Item | Priority |
|---|---|
| `src/provider/mod.rs` `fn print_command` | Medium |
| `src/commands/start.rs` `fn run` | Medium |
| `src/commands/rebuild.rs` `fn run` | Medium |
| `src/lib.rs` module-level `//!` comment | Medium |
| `src/provider/docker.rs` `struct Docker` | Low |
| `src/provider/docker.rs` `BuildSource` variants | Low |
| `src/provider/podman.rs` `struct Podman` | Low |
| `src/provider/docker_compose.rs` `struct DockerCompose` | Low |
| `src/provider/podman_compose.rs` `struct PodmanCompose` | Low |
| `src/devcontainers/config.rs` `struct Config` fields | Low |
| `src/devcontainers/config.rs` `struct Build` fields | Low |
| `src/devcontainers/mod.rs` `fn exec_host_hook` | Low |

## Goal

Add the missing doc comments. Prioritise Medium items first. `# Errors` sections required for
functions that return `Result`.

## Functional Requirements

- FR-001: `src/lib.rs` gains a crate-level `//!` comment explaining the library surface.
- FR-002: `print_command`, `start::run`, `rebuild::run` gain `///` doc comments with
  `# Errors` sections.
- FR-003: Provider structs (`Docker`, `Podman`, `DockerCompose`, `PodmanCompose`) gain one-line
  struct-level `///` doc comments.
- FR-004: `BuildSource` variants gain `///` doc comments.
- FR-005: `Config` and `Build` fields gain per-field `///` doc comments.
- FR-006: `exec_host_hook` gains a `///` doc comment (can be `pub(crate)` level).
- FR-007: `cargo doc --no-deps` must pass with no warnings.

## Out of Scope

- Adding docs to private implementation details beyond `exec_host_hook`.
- Writing a man page (future).
