# Plan: Podman Provider Integration Tests

## Phase 1: Podman Installation

- [x] Task: Install Podman and podman-compose in dev container
    - [x] Add `podman` and `podman-compose` to apt-get install in
          `.devcontainer/Dockerfile`
    - [x] Add `/etc/subuid` and `/etc/subgid` entries for the `devcont` user
          to enable rootless Podman
    - [ ] Verify `podman --version` and `podman-compose --version` run inside
          the dev container after rebuild
- [x] Task: Add Podman installation step to CI
    - [x] In `.github/workflows/test.yml`, add an apt install step for
          `podman` and `podman-compose` before the integration test step
    - [ ] Verify `podman --version` is available on `ubuntu-latest`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Podman Installation' (Protocol in workflow.md)

## Phase 2: Podman Provider Tests (RED)

- [x] Task: Write failing tests for Podman provider lifecycle (RED)
    - [x] `test_podman_exists_returns_false_before_create()` — assert
          `provider.exists()` is `false` for an unknown container name
    - [x] `test_podman_build_and_create()` — build image and create container,
          assert `exists()` returns `true`
    - [x] `test_podman_start_and_running()` — start container, assert
          `running()` returns `true`
    - [x] `test_podman_running_returns_false_when_stopped()` — stop container,
          assert `running()` returns `false`
    - [x] `test_podman_restart()` — restart running container, assert still
          `running()`
    - [x] `test_podman_exec()` — exec `echo hello` inside container, assert
          success
    - [x] `test_podman_cp()` — copy a temp file into the container, exec
          `test -f <dest>` to confirm presence
    - [x] `test_podman_stop_and_rm()` — stop and rm, assert `exists()` is
          `false`
    - [x] Confirm all tests fail (no implementation changes)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Podman Provider Tests (RED)' (Protocol in workflow.md)

## Phase 3: Podman Provider Tests (GREEN)

- [x] Task: Add `load_podman_provider` test helper
    - [x] Constructs a `Podman` struct directly with a unique name and the
          `basic` fixture Dockerfile (same fixture as Docker track)
    - [x] Uses `devcont-itest-podman-<ts>` naming prefix
- [x] Task: Extend RAII guard for Podman cleanup
    - [x] Add a `PodmanGuard` (or extend `ContainerGuard`) that calls
          `podman rm -f <name>` and `podman rmi <image>` on drop
- [x] Task: Make Podman lifecycle tests pass (GREEN)
    - [x] Run each test; fix any rootless-specific issues
          (subuid/subgid, userns, security labels)
    - [x] All 8 Podman tests pass with `cargo test --test integration`
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Podman Provider Tests (GREEN)' (Protocol in workflow.md)

## Phase 4: PodmanCompose Provider Tests (RED → GREEN)

- [x] Task: Write failing tests for PodmanCompose provider (RED)
    - [x] `test_podman_compose_exists_returns_false_before_build()`
    - [x] `test_podman_compose_build_and_start()` — build and start project,
          assert `exists()` and `running()` return `true`
    - [x] `test_podman_compose_exec()` — exec a command in the service
    - [x] `test_podman_compose_cp()` — copy a file into the service container
    - [x] `test_podman_compose_restart()` — restart, assert still `running()`
    - [x] `test_podman_compose_stop_and_rm()` — down the project, assert
          `exists()` returns `false`
    - [x] Confirm all tests fail
- [x] Task: Add `load_podman_compose_provider` test helper
    - [x] Constructs a `PodmanCompose` struct directly with the compose fixture
          (reused from Docker track) and a unique project name
- [x] Task: Extend RAII guard for PodmanCompose cleanup
    - [x] Add teardown that runs `podman-compose -p <name> down
          --remove-orphans` on drop
- [x] Task: Make PodmanCompose tests pass (GREEN)
    - [x] Fix any podman-compose specific issues (SSH agent socket path, temp
          compose file generation)
    - [x] All 6 PodmanCompose tests pass
- [ ] Task: Conductor - User Manual Verification 'Phase 4: PodmanCompose Provider Tests (RED → GREEN)' (Protocol in workflow.md)

## Phase 5: Quality Gate

- [x] Task: Run full quality gate
    - [x] `cargo test` (unit tests) ✓
    - [x] `cargo test --test integration` ✓ (Docker + Podman, 32/32)
    - [x] `cargo clippy --all-targets -- -D warnings` ✓
    - [x] `cargo fmt --check` ✓
    - [x] `cargo deny check licenses bans advisories` ✓
    - [x] Confirm no orphan containers: `podman ps -a --filter name=devcont-itest-podman` returns empty
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Quality Gate' (Protocol in workflow.md)
