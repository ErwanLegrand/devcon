# Spec: Tests — PodmanCompose Provider — Integration and Unit Coverage

## Problem

`src/provider/podman_compose.rs` has partial unit test coverage (running_args, cp, rm added in
a prior session), but `build()`, `create()`, `exists()`, `start()`, `stop()`, `restart()`,
`attach()`, `exec()`, `exec_raw()` have no unit test coverage. The file is one of the most
complex in the codebase and has multiple known bugs (mounts parity, exists()).

Baseline coverage: `podman_compose.rs` ~35%.

## Goal

Expand unit test coverage for `PodmanCompose` to ≥70% line coverage, focusing on argument
construction and output parsing.

## Functional Requirements

- FR-001: Tests are unit tests (no running container runtime required). Use command argument
  inspection where possible (methods that return `Vec<String>` of args).
- FR-002: Cover `create()` args: name, image, workspace, env vars, mounts, run args.
- FR-003: Cover `exists()` parsing: valid compose ps output → true, empty output → false.
- FR-004: Cover `exec()` and `exec_raw()` arg construction.
- FR-005: Cover `attach()` shell selection.
- FR-006: Cover compose override YAML generation via `create_compose_override`.
- FR-007: `cargo test` must pass.
- FR-008: `cargo llvm-cov` shows ≥70% for `podman_compose.rs`.

## Out of Scope

- Integration tests requiring podman-compose to be installed (separate integration test track).
- Testing the container runtime interactions end-to-end.
