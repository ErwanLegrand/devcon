# Spec: Fix — Propagate Non-Zero Exit From In-Container Lifecycle Hooks

## Problem

`exec_hook` returns `Result<bool>` where the `bool` indicates success, but the callers in `run()`
and `rebuild()` do not always check this bool and abort on `false`. Concretely:

- `postCreateCommand` failure currently does not abort the lifecycle — the container continues
  attaching even if setup failed.
- `postStartCommand` and `postAttachCommand` have the same issue.

The host-side hook (`exec_host_hook` / `initializeCommand`) was already fixed to abort on
non-zero exit. The in-container hooks need the same treatment.

Note: `exec_host_hook` fix was applied in a prior session. This track covers the container-side
hooks only.

## Goal

Ensure that a non-zero exit from any in-container lifecycle hook (`postCreateCommand`,
`postStartCommand`, `postAttachCommand`) aborts the current operation with a descriptive error.

## Functional Requirements

- FR-001: In `Devcontainer::run()`, check the return value of `exec_hook` for each in-container
  lifecycle hook. If `Ok(false)`, return `Err` with the hook name in the message.
- FR-002: In `Devcontainer::rebuild()`, apply the same check for all hooks called there.
- FR-003: The error message must name the failing hook, e.g.:
  `"postCreateCommand failed with non-zero exit"`.
- FR-004: Unit tests: mock provider returning `Ok(false)` for exec causes run() to error.
- FR-005: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Changing how `exec_host_hook` works (already fixed).
- Adding timeout to hooks (tracked as `feature_hook_timeout`).
