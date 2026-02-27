# Plan: Podman Provider Integration Tests

## Phase 1: Podman Installation

- [ ] Task: Install Podman and podman-compose in dev container
    - [ ] Add `podman` and `podman-compose` to apt-get install in
          `.devcontainer/Dockerfile`
    - [ ] Add `/etc/subuid` and `/etc/subgid` entries for the `devcont` user
          to enable rootless Podman
    - [ ] Verify `podman --version` and `podman-compose --version` run inside
          the dev container after rebuild
- [ ] Task: Add Podman installation step to CI
    - [ ] In `.github/workflows/test.yml`, add an apt install step for
          `podman` and `podman-compose` before the integration test step
    - [ ] Verify `podman --version` is available on `ubuntu-latest`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Podman Installation' (Protocol in workflow.md)

## Phase 2: Podman Provider Tests (RED)

- [ ] Task: Write failing tests for Podman provider lifecycle (RED)
    - [ ] `test_podman_exists_returns_false_before_create()` — assert
          `provider.exists()` is `false` for an unknown container name
    - [ ] `test_podman_build_and_create()` — build image and create container,
          assert `exists()` returns `true`
    - [ ] `test_podman_start_and_running()` — start container, assert
          `running()` returns `true`
    - [ ] `test_podman_running_returns_false_when_stopped()` — stop container,
          assert `running()` returns `false`
    - [ ] `test_podman_restart()` — restart running container, assert still
          `running()`
    - [ ] `test_podman_exec()` — exec `echo hello` inside container, assert
          success
    - [ ] `test_podman_cp()` — copy a temp file into the container, exec
          `test -f <dest>` to confirm presence
    - [ ] `test_podman_stop_and_rm()` — stop and rm, assert `exists()` is
          `false`
    - [ ] Confirm all tests fail (no implementation changes)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Podman Provider Tests (RED)' (Protocol in workflow.md)

## Phase 3: Podman Provider Tests (GREEN)

- [ ] Task: Add `load_podman_provider` test helper
    - [ ] Constructs a `Podman` struct directly with a unique name and the
          `basic` fixture Dockerfile (same fixture as Docker track)
    - [ ] Uses `devcont-itest-podman-<ts>` naming prefix
- [ ] Task: Extend RAII guard for Podman cleanup
    - [ ] Add a `PodmanGuard` (or extend `ContainerGuard`) that calls
          `podman rm -f <name>` and `podman rmi <image>` on drop
- [ ] Task: Make Podman lifecycle tests pass (GREEN)
    - [ ] Run each test; fix any rootless-specific issues
          (subuid/subgid, userns, security labels)
    - [ ] All 8 Podman tests pass with `cargo test --test integration`
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Podman Provider Tests (GREEN)' (Protocol in workflow.md)

## Phase 4: PodmanCompose Provider Tests (RED → GREEN)

- [ ] Task: Write failing tests for PodmanCompose provider (RED)
    - [ ] `test_podman_compose_exists_returns_false_before_build()`
    - [ ] `test_podman_compose_build_and_start()` — build and start project,
          assert `exists()` and `running()` return `true`
    - [ ] `test_podman_compose_exec()` — exec a command in the service
    - [ ] `test_podman_compose_cp()` — copy a file into the service container
    - [ ] `test_podman_compose_restart()` — restart, assert still `running()`
    - [ ] `test_podman_compose_stop_and_rm()` — down the project, assert
          `exists()` returns `false`
    - [ ] Confirm all tests fail
- [ ] Task: Add `load_podman_compose_provider` test helper
    - [ ] Constructs a `PodmanCompose` struct directly with the compose fixture
          (reused from Docker track) and a unique project name
- [ ] Task: Extend RAII guard for PodmanCompose cleanup
    - [ ] Add teardown that runs `podman-compose -p <name> down
          --remove-orphans` on drop
- [ ] Task: Make PodmanCompose tests pass (GREEN)
    - [ ] Fix any podman-compose specific issues (SSH agent socket path, temp
          compose file generation)
    - [ ] All 6 PodmanCompose tests pass
- [ ] Task: Conductor - User Manual Verification 'Phase 4: PodmanCompose Provider Tests (RED → GREEN)' (Protocol in workflow.md)

## Phase 5: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test` (unit tests) ✓
    - [ ] `cargo test --test integration` ✓ (Docker + Podman)
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
    - [ ] Confirm no orphan containers: `podman ps -a --filter name=devcont-itest-podman` returns empty
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Quality Gate' (Protocol in workflow.md)
