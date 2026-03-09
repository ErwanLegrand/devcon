# Spec: Fix — Container Name Filter in exists() and running() is Too Broad

## Problem

`Docker::exists()` and `Podman::exists()` call `docker ps -a --filter name=<name>` and check
whether the output is non-empty. However, `--filter name=` is a **substring match**, not an
exact match. A container named `myproject` would also match a filter for `my`. This means
`exists()` can return `true` for the wrong container, causing incorrect `start`/`rm` behavior.

The same issue applies to `running()`.

## Goal

Ensure `exists()` and `running()` match the exact container name.

## Functional Requirements

- FR-001: Use `--filter name=^/mycontainer$` (anchored regex) or parse the output and compare
  the name column exactly.
- FR-002: Apply the fix to Docker, Podman, DockerCompose, and PodmanCompose providers.
- FR-003: Unit tests: a filter for `"foo"` does not match `"foobar"` or `"barfoo"`.
- FR-004: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Changing the `safe_name()` implementation (tracked in `fix_safe_name_validation`).
