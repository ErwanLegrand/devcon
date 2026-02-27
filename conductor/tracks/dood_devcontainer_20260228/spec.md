# Spec: Docker-outside-of-Docker for Dev Container

## Overview

Add Docker-outside-of-Docker (DooD) support to the project's dev container so
that the `devcont` binary can be exercised against real container workflows
from inside the dev container. Add a minimal integration test suite in
`tests/integration.rs` that verifies core `devcont` commands end-to-end using
small purpose-built fixture devcontainer configs.

## Functional Requirements

### FR-001: Docker CLI in Dockerfile
`.devcontainer/Dockerfile` must install the Docker CLI so that `docker`
commands execute inside the dev container.

### FR-002: Socket Bind-Mount
`devcontainer.json` must bind-mount `/var/run/docker.sock` from host into the
container at the same path, connecting the CLI to the host daemon.

### FR-003: Socket Group Permissions
The `devcont` user must be added to the `docker` group so it can access the
socket without `sudo`.

### FR-004: Minimal Integration Test Fixtures
Add purpose-built fixtures under `tests/fixtures/integration/`:
- `basic/` — `FROM alpine:latest` Dockerfile + minimal `devcontainer.json`
- `post_create/` — same base, plus a `postCreateCommand` to verify lifecycle
  hooks are invoked

### FR-005: Integration Test Binary
Add `tests/integration.rs` with tests that:
1. Build and create a container from a fixture via the `Devcontainer` API
2. Assert the container exists and is running
3. Execute a command inside the container and verify it succeeds
4. Stop and remove the container (cleanup in both pass and fail paths)

### FR-006: CI Execution
`test.yml` must run `cargo test --test integration` after unit tests. Docker is
available natively on `ubuntu-latest` runners; no DinD setup needed in CI.

## Non-Functional Requirements

- Test containers must use a unique name prefix (`devcont-itest-`) to avoid
  collisions with user containers
- Cleanup must run even on test failure (use a RAII guard or explicit teardown)
- No `--privileged` flag anywhere
- `cargo test` (unit tests only) must remain fast and unaffected

## Acceptance Criteria

- `docker ps` runs successfully inside the dev container without `sudo`
- `cargo test --test integration` passes inside the dev container
- `cargo test --test integration` passes in GitHub Actions CI
- No orphan `devcont-itest-*` containers remain after a test run
- `cargo test` (unit tests) is unaffected in speed and output

## Out of Scope

- True DinD / `--privileged` mode
- Official devcontainers sample repos as test fixtures
- Podman socket support (planned for a future integration test expansion track)
- Interactive `attach` testing (requires a TTY)
- A nightly compatibility suite against real-world devcontainer configs
- Expanded integration tests against Docker and Podman providers (future track)
