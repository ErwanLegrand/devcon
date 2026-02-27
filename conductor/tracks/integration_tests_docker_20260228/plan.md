# Plan: Docker Provider Integration Tests

## Phase 1: Compose Fixture and Guard Extension

- [ ] Task: Add compose integration fixture
    - [ ] Create `tests/fixtures/integration/compose/docker-compose.yml`
          with a single `app` service (`image: alpine:latest`,
          `command: sleep infinity`)
    - [ ] Create `tests/fixtures/integration/compose/devcontainer.json`
          with `dockerComposeFile`, `service: "app"`,
          `workspaceFolder: "/workspace"`
- [ ] Task: Extend RAII ContainerGuard for compose teardown
    - [ ] Add a `ComposeGuard` variant (or extend `ContainerGuard`) that runs
          `docker compose -p <name> down --remove-orphans --rmi all` on drop
    - [ ] Write a test that verifies `ComposeGuard::drop()` removes the project
          (RED → GREEN)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Compose Fixture and Guard Extension' (Protocol in workflow.md)

## Phase 2: Docker Provider Tests (RED)

- [ ] Task: Write failing tests for Docker provider lifecycle (RED)
    - [ ] `test_docker_exists_returns_false_before_create()` — assert
          `provider.exists()` is `false` for an unknown container name
    - [ ] `test_docker_build_and_create()` — build image and create container,
          assert `exists()` returns `true`
    - [ ] `test_docker_start_and_running()` — start container, assert
          `running()` returns `true`
    - [ ] `test_docker_running_returns_false_when_stopped()` — stop container,
          assert `running()` returns `false`
    - [ ] `test_docker_restart()` — restart running container, assert still
          `running()`
    - [ ] `test_docker_exec()` — exec `echo hello` inside container, assert
          success
    - [ ] `test_docker_cp()` — copy a temp file into the container, exec
          `test -f <dest>` to confirm presence
    - [ ] `test_docker_stop_and_rm()` — stop and rm container, assert
          `exists()` returns `false`
    - [ ] Confirm all tests fail (no implementation changes)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Docker Provider Tests (RED)' (Protocol in workflow.md)

## Phase 3: Docker Provider Tests (GREEN)

- [ ] Task: Add `load_docker_provider` test helper
    - [ ] Add helper in `tests/integration.rs` that constructs a `Docker`
          struct directly (bypassing `Devcontainer::load`) with a given name,
          pointing to the `basic` fixture Dockerfile
    - [ ] Verify helper compiles and is reachable from tests
- [ ] Task: Make Docker lifecycle tests pass (GREEN)
    - [ ] Run each test in sequence; fix any provider or fixture issues
    - [ ] Ensure `ContainerGuard` cleans up between tests
    - [ ] All 8 Docker tests pass with `cargo test --test integration`
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Docker Provider Tests (GREEN)' (Protocol in workflow.md)

## Phase 4: DockerCompose Provider Tests (RED → GREEN)

- [ ] Task: Write failing tests for DockerCompose provider (RED)
    - [ ] `test_compose_exists_returns_false_before_build()`
    - [ ] `test_compose_build_and_start()` — build and start compose project,
          assert `exists()` and `running()` return `true`
    - [ ] `test_compose_exec()` — exec a command in the service container
    - [ ] `test_compose_cp()` — copy a file into the service container
    - [ ] `test_compose_restart()` — restart service, assert still `running()`
    - [ ] `test_compose_stop_and_rm()` — stop and down the project,
          assert `exists()` returns `false`
    - [ ] Confirm all tests fail
- [ ] Task: Add `load_compose_provider` test helper
    - [ ] Constructs a `DockerCompose` struct directly with the compose fixture
          path and a unique project name
- [ ] Task: Make DockerCompose tests pass (GREEN)
    - [ ] Run each test; fix any provider or fixture issues
    - [ ] Ensure `ComposeGuard` cleans up between tests
    - [ ] All 6 DockerCompose tests pass
- [ ] Task: Conductor - User Manual Verification 'Phase 4: DockerCompose Provider Tests (RED → GREEN)' (Protocol in workflow.md)

## Phase 5: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test` (unit tests) ✓
    - [ ] `cargo test --test integration` ✓
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
    - [ ] Confirm no orphan containers: `docker ps -a --filter name=devcont-itest-docker` returns empty
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Quality Gate' (Protocol in workflow.md)
