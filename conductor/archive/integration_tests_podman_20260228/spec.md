# Spec: Podman Provider Integration Tests

## Overview

Comprehensive integration tests for the `Podman` and `PodmanCompose` provider
implementations, covering all `Provider` trait methods. Requires Podman and
podman-compose installed in the dev container and in CI. Follows patterns and
reuses fixtures established by the Docker integration tests track.

**Depends on:** `dood_devcontainer_20260228`, `integration_tests_docker_20260228`

## Functional Requirements

### FR-001: Podman in dev container
`.devcontainer/Dockerfile` must install `podman` and `podman-compose`.
The `devcont` user must have valid `/etc/subuid` and `/etc/subgid` entries for
rootless Podman operation.

### FR-002: Podman in CI
`.github/workflows/test.yml` must install `podman` and `podman-compose` via
apt before the integration test step. Both are available in the default Ubuntu
package repositories on `ubuntu-latest`.

### FR-003: Podman provider — full method coverage
Integration tests for every `Podman` `Provider` method:
`build`, `create`, `start`, `stop`, `restart`, `rm`, `exists`, `running`,
`cp`, `exec`. (`attach` excluded — requires an interactive TTY.)

Each method must have at least one happy-path test. `exists` and `running`
must also have negative-path tests.

### FR-004: PodmanCompose provider — full method coverage
Integration tests for every `PodmanCompose` `Provider` method. The compose
fixture from `integration_tests_docker_20260228` is reused or adapted
(replacing `docker compose` semantics with `podman-compose` where needed).

### FR-005: Rootless Podman compatibility
All fixtures must be compatible with rootless Podman:
- Use `overrideCommand: true` in `devcontainer.json`
- The `Podman` provider already passes `--security-opt label=disable` and
  `--userns=keep-id`; tests must not contradict these flags

### FR-006: Test isolation and cleanup
- Unique project/container names: `devcont-itest-podman-<ts>`
- RAII guard extended to handle `podman rm -f` and
  `podman-compose -p <name> down --remove-orphans`

## Non-Functional Requirements

- Rootless Podman must not require `--privileged` or root access
- Tests must clean up images after the run
- Total added runtime should remain under 3 minutes

## Acceptance Criteria

- `podman --version` and `podman-compose --version` succeed in the dev container
- All 10 `Podman` `Provider` methods covered (excluding `attach`)
- All 10 `PodmanCompose` `Provider` methods covered (excluding `attach`)
- Negative-path tests for `exists` and `running`
- No orphan containers or images remain after the test run
- `cargo test --test integration` passes in GitHub Actions CI

## Out of Scope

- Docker provider (covered by `integration_tests_docker_20260228`)
- `attach` method
- Privileged / rootful Podman
- `podman machine` (macOS VM layer — Linux-only for now)
