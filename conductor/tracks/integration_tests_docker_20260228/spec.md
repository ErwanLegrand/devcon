# Spec: Docker Provider Integration Tests

## Overview

Comprehensive integration tests for the `Docker` and `DockerCompose` provider
implementations, covering all `Provider` trait methods against a live Docker
daemon. Builds on the infrastructure (fixtures, RAII guard, `load_fixture`
helper) established by the `dood_devcontainer_20260228` track.

**Depends on:** `dood_devcontainer_20260228`

## Functional Requirements

### FR-001: Docker provider — full method coverage
Add integration tests for every `Docker` `Provider` method:
`build`, `create`, `start`, `stop`, `restart`, `rm`, `exists`, `running`,
`cp`, `exec`. (`attach` excluded — requires an interactive TTY.)

Each method must have at least one test asserting the happy path. Critical
methods (`exists`, `running`, `exec`, `cp`) must also have a negative-path
test (e.g., `exists` returns `false` before creation).

### FR-002: DockerCompose provider — full method coverage
Add integration tests for every `DockerCompose` `Provider` method using the
compose fixture, with the same coverage target as FR-001.

### FR-003: Compose fixture
Add `tests/fixtures/integration/compose/` containing:
- `docker-compose.yml` — single service named `app` based on `alpine:latest`,
  with `command: sleep infinity` so the container stays alive during tests
- `devcontainer.json` — `dockerComposeFile: "docker-compose.yml"`,
  `service: "app"`, `workspaceFolder: "/workspace"`

### FR-004: Test isolation and cleanup
- Each test allocates a unique project/container name via the `unique_name`
  helper from the DooD track (`devcont-itest-docker-<ts>`)
- The RAII `ContainerGuard` from the DooD track is extended to handle compose
  teardown: runs `docker compose -p <name> down --remove-orphans --rmi all`
- Tests must not share state; each test is self-contained

### FR-005: No new CI step required
All new tests reside in `tests/integration.rs` and are covered by the existing
`cargo test --test integration` step added in the DooD track.

## Non-Functional Requirements

- Tests must clean up images created during the test run to avoid filling disk
- Total integration test runtime should remain under 3 minutes
- No hardcoded container names or image tags

## Acceptance Criteria

- All 10 `Docker` `Provider` methods covered (excluding `attach`)
- All 10 `DockerCompose` `Provider` methods covered (excluding `attach`)
- Negative-path tests for `exists` and `running`
- No orphan containers, networks, or images after the test run
- `cargo test --test integration` passes on a machine with Docker available

## Out of Scope

- Podman provider (separate track: `integration_tests_podman_20260228`)
- `attach` method (requires interactive TTY)
- Port forwarding runtime verification
- Error-path tests for Docker daemon unavailability
