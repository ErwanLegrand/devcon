# Plan: Docker-outside-of-Docker for Dev Container

## Phase 1: Dev Container Wiring

- [ ] Task: Install Docker CLI in Dockerfile
    - [ ] Add `docker.io` to apt-get install in `.devcontainer/Dockerfile`
    - [ ] Add `devcont` user to `docker` group via `usermod -aG docker $USER`
    - [ ] Verify `docker --version` runs as the `devcont` user in a test build
- [ ] Task: Add socket bind-mount to devcontainer.json
    - [ ] Add `"source=/var/run/docker.sock,target=/var/run/docker.sock,type=bind"` to `mounts`
    - [ ] Verify `docker ps` works inside the dev container after rebuild
- [ ] Task: Update post-create.sh
    - [ ] Add a check that prints a warning if `/var/run/docker.sock` is not
          accessible (host may not have Docker running)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Dev Container Wiring' (Protocol in workflow.md)

## Phase 2: Integration Test Infrastructure

- [ ] Task: Write a failing test harness (RED) before any fixtures exist
    - [ ] Create `tests/integration.rs` with a single `#[test] fn placeholder()`
          that asserts `false` with message "not yet implemented"
    - [ ] Confirm `cargo test --test integration` fails as expected
- [ ] Task: Create integration test fixtures
    - [ ] Create `tests/fixtures/integration/basic/Dockerfile` with `FROM alpine:latest`
          and `CMD ["sh"]`
    - [ ] Create `tests/fixtures/integration/basic/devcontainer.json` with
          `name`, `build.dockerfile`, `remoteUser`, `overrideCommand: true`
    - [ ] Create `tests/fixtures/integration/post_create/Dockerfile` (same base)
    - [ ] Create `tests/fixtures/integration/post_create/devcontainer.json` with
          a `postCreateCommand` that writes a marker file
- [ ] Task: Add RAII cleanup guard
    - [ ] Create a `ContainerGuard` struct in `tests/integration.rs` that calls
          `docker rm -f <name>` in its `Drop` impl to ensure cleanup on panic
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration Test Infrastructure' (Protocol in workflow.md)

## Phase 3: Integration Tests

- [ ] Task: Write failing test — basic build and create (RED)
    - [ ] Add `test_basic_build_and_create()`: loads the `basic` fixture,
          calls `provider.build()` and `provider.create()`, asserts container
          exists via `provider.exists()`
    - [ ] Confirm test fails (no implementation changes yet)
- [ ] Task: Write failing test — exec inside container (RED)
    - [ ] Add `test_exec_in_container()`: starts container, calls
          `provider.exec("echo hello")`, asserts success
- [ ] Task: Write failing test — post_create lifecycle hook (RED)
    - [ ] Add `test_post_create_command()`: runs full `Devcontainer::run()` on
          the `post_create` fixture, asserts the marker file exists in the container
- [ ] Task: Implement test helpers (GREEN)
    - [ ] Add `load_fixture(name: &str) -> Devcontainer` helper that resolves
          `tests/fixtures/integration/<name>` and calls `Devcontainer::load()`
    - [ ] Add `unique_name(prefix: &str) -> String` that appends a timestamp
          to produce a unique `devcont-itest-<prefix>-<ts>` container name
    - [ ] Verify all three tests pass with a live Docker daemon
- [ ] Task: Verify cleanup — no orphan containers remain after test run
    - [ ] Run `cargo test --test integration` twice in sequence
    - [ ] Assert `docker ps -a --filter name=devcont-itest` returns empty
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration Tests' (Protocol in workflow.md)

## Phase 4: CI Integration

- [ ] Task: Update `test.yml` to run integration tests
    - [ ] Add step: `cargo test --test integration --verbose` after unit tests
    - [ ] Confirm Docker is available on `ubuntu-latest` without extra setup
- [ ] Task: Run full CI quality gate locally
    - [ ] `cargo test` ✓
    - [ ] `cargo test --test integration` ✓
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
- [ ] Task: Conductor - User Manual Verification 'Phase 4: CI Integration' (Protocol in workflow.md)
