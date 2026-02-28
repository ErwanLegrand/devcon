# Plan: Docker-outside-of-Docker for Dev Container

## Phase 1: Dev Container Wiring

- [x] Task: Install Docker CLI in Dockerfile
    - [x] Add `docker.io` to apt-get install in `.devcontainer/Dockerfile`
    - [x] Add `devcont` user to `docker` group via `usermod -aG docker $USER`
    - [x] Verify `docker --version` runs as the `devcont` user in a test build
- [x] Task: Add socket bind-mount to devcontainer.json
    - [x] Add `"source=/var/run/docker.sock,target=/var/run/docker.sock,type=bind"` to `mounts`
    - [x] Verify `docker ps` works inside the dev container after rebuild
- [x] Task: Update post-create.sh
    - [x] Add a check that prints a warning if `/var/run/docker.sock` is not
          accessible (host may not have Docker running)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Dev Container Wiring' (Protocol in workflow.md)

## Phase 2: Integration Test Infrastructure

- [x] Task: Write a failing test harness (RED) before any fixtures exist
    - [x] Create `tests/integration.rs` with a single `#[test] fn placeholder()`
          that asserts `false` with message "not yet implemented"
    - [x] Confirm `cargo test --test integration` fails as expected
- [x] Task: Create integration test fixtures
    - [x] Create `tests/fixtures/integration/basic/Dockerfile` with `FROM alpine:latest`
          and `CMD ["sh"]`
    - [x] Create `tests/fixtures/integration/basic/devcontainer.json` with
          `name`, `build.dockerfile`, `remoteUser`, `overrideCommand: true`
    - [x] Create `tests/fixtures/integration/post_create/Dockerfile` (same base)
    - [x] Create `tests/fixtures/integration/post_create/devcontainer.json` with
          a `postCreateCommand` that writes a marker file
- [x] Task: Add RAII cleanup guard
    - [x] Create a `ContainerGuard` struct in `tests/integration.rs` that calls
          `docker rm -f <name>` in its `Drop` impl to ensure cleanup on panic
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration Test Infrastructure' (Protocol in workflow.md)

## Phase 3: Integration Tests

- [x] Task: Write failing test — basic build and create (RED)
    - [x] Add `test_basic_build_and_create()`: loads the `basic` fixture,
          calls `provider.build()` and `provider.create()`, asserts container
          exists via `provider.exists()`
    - [x] Confirm test fails (no implementation changes yet)
- [x] Task: Write failing test — exec inside container (RED)
    - [x] Add `test_exec_in_container()`: starts container, calls
          `provider.exec("echo hello")`, asserts success
- [x] Task: Write failing test — post_create lifecycle hook (RED)
    - [x] Add `test_post_create_command()`: runs full `Devcontainer::run()` on
          the `post_create` fixture, asserts the marker file exists in the container
- [x] Task: Implement test helpers (GREEN)
    - [x] Add `load_fixture(name: &str) -> Devcontainer` helper that resolves
          `tests/fixtures/integration/<name>` and calls `Devcontainer::load()`
    - [x] Add `unique_name(prefix: &str) -> String` that appends a timestamp
          to produce a unique `devcont-itest-<prefix>-<ts>` container name
    - [x] Verify all three tests pass with a live Docker daemon
- [x] Task: Verify cleanup — no orphan containers remain after test run
    - [x] Run `cargo test --test integration` twice in sequence
    - [x] Assert `docker ps -a --filter name=devcont-itest` returns empty
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration Tests' (Protocol in workflow.md)

## Phase 4: CI Integration

- [x] Task: Update `test.yml` to run integration tests
    - [x] Add step: `cargo test --test integration --verbose` after unit tests
    - [x] Confirm Docker is available on `ubuntu-latest` without extra setup
- [ ] Task: Run full CI quality gate locally
    - [ ] `cargo test` ✓
    - [ ] `cargo test --test integration` ✓
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
- [ ] Task: Conductor - User Manual Verification 'Phase 4: CI Integration' (Protocol in workflow.md)
