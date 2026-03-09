# Test Coverage Assessment Report
## date: 2026-03-09

## 1. Baseline Coverage

Coverage measured with `cargo llvm-cov --summary-only` (line coverage).

| File | Line Coverage % | Status |
|---|---|---|
| `src/commands/rebuild.rs` | 0.00% | ⚠️ <80% |
| `src/commands/start.rs` | 0.00% | ⚠️ <80% |
| `src/devcontainers/config.rs` | 80.43% | ✅ ≥80% |
| `src/devcontainers/mod.rs` | 18.16% | ⚠️ <80% |
| `src/devcontainers/one_or_many.rs` | 88.89% | ✅ ≥80% |
| `src/main.rs` | 45.45% | ⚠️ <80% |
| `src/provider/docker.rs` | 73.60% | ⚠️ <80% |
| `src/provider/docker_compose.rs` | 77.97% | ⚠️ <80% |
| `src/provider/mod.rs` | 100.00% | ✅ ≥80% |
| `src/provider/podman.rs` | 78.07% | ⚠️ <80% |
| `src/provider/podman_compose.rs` | 37.81% | ⚠️ <80% |
| `src/provider/utils.rs` | 86.05% | ✅ ≥80% |
| `src/settings.rs` | 68.75% | ⚠️ <80% |

**Overall line coverage: 54.32%** (target: 80%)

### Coverage notes

- Integration tests (`tests/integration.rs`) exercise Docker and Podman provider methods at the process level (build, create, start, stop, restart, exec, cp, rm, exists, running). These account for most of the provider line coverage.
- Unit tests exist for `exec_hook`/`exec_hook_many` in `devcontainers/mod.rs` and for argument-building helpers in `provider/podman_compose.rs`, but the `Devcontainer` struct methods (`run`, `rebuild`, `post_create`, `create`, `copy`, `copy_gitconfig`, `copy_dotfiles`, `create_args`) have essentially no unit-test coverage.
- `provider/utils.rs` is covered at 86% via integration tests that exercise `create_compose_override` indirectly; the SSH-agent-absent branch has no dedicated test.
- The `commands/` layer is 0% covered — it is a thin CLI shim, but `get_project_directory` has non-trivial env-expansion logic.

---

## 2. Gap Inventory

### `src/devcontainers/mod.rs`

#### Uncovered: `Devcontainer::run` (lines L101–L135)
- **Gap type**: No unit test
- **Risk**: High — `run()` orchestrates the entire container lifecycle: initialize hook, create, start, post-start hook, post-create, restart, attach, post-attach hook, and optional shutdown. A bug in any branch (e.g., wrong hook dispatch order, shutdown guard not firing, `initializeCommand` failure not propagating) silently breaks the primary user workflow.
- **Feasibility**: Unit test possible — the `MockProvider` already exists in the same module. A test only needs to construct a `Devcontainer` with a `Config` fixture and inject the mock provider. The lifecycle sequence and each optional hook path (Some/None) can be validated without a real container.
- **Sketch**: Build a `Devcontainer` via `Devcontainer { config, provider: Box::new(MockProvider::new()), settings }`, call `run(true)`, and assert that the mock recorded the correct sequence of calls (build → create → start → exec hooks → restart → attach → stop iff `should_shutdown()`). Repeat with various hook `Some`/`None` combinations and with `initializeCommand` returning `false`.

#### Uncovered: `Devcontainer::rebuild` (lines L141–L149)
- **Gap type**: No unit test
- **Risk**: Medium — `rebuild` conditionally calls `stop` + `rm` before delegating to `run`. The guard condition `provider.exists()?` is the critical path. If `exists()` or `rm()` silently succeeds when it should not, the container state is corrupted.
- **Feasibility**: Unit test possible — `MockProvider::exists` can be configured to return `true` or `false`; assert `stop` and `rm` are (or are not) invoked.
- **Sketch**: Two cases: (1) `exists()` → `true`: assert `stop` + `rm` called, then `run` proceeds; (2) `exists()` → `false`: assert neither `stop` nor `rm` called.

#### Uncovered: `Devcontainer::create` (lines L151–L160)
- **Gap type**: No unit test
- **Risk**: Medium — The `!provider.exists()` guard means `build` + `create` are skipped on a running container. Inversion of this condition would rebuild every time, destroying data.
- **Feasibility**: Unit test possible with `MockProvider`.
- **Sketch**: (1) `exists()` → `true`: assert `build` and `create` not called; (2) `exists()` → `false`: assert both called.

#### Uncovered: `Devcontainer::post_create` (lines L162–L181)
- **Gap type**: No unit test
- **Risk**: Medium — Each lifecycle hook (`on_create_command`, `update_content_command`, `post_create_command`) may be absent. If the dispatch logic has a regression (e.g., wrong hook fired), users silently miss setup steps. `copy_gitconfig` and `copy_dotfiles` are also called here.
- **Feasibility**: Unit test possible — verify that each hook fires via mock `exec`/`exec_raw` calls.
- **Sketch**: Populate config with each hook set to `Some(OneOrMany::One(...))` in turn; assert `MockProvider::exec_calls` contains the expected command string.

#### Uncovered: `Devcontainer::copy` / `copy_gitconfig` / `copy_dotfiles` (lines L183–L242)
- **Gap type**: No unit test (filesystem-dependent)
- **Risk**: Medium — `copy` shell-quotes the destination path to guard against special characters. A quoting bug breaks `mkdir -p` inside the container for paths with spaces or apostrophes. `copy_dotfiles` iterates `settings.dotfiles` and resolves `~`; an empty list silently succeeds.
- **Feasibility**: Unit test feasible for path-quoting logic and the `remote_user == "root"` vs non-root home directory selection. The `source.exists()` check requires a real file on disk, but `tempfile` or an existing fixture can serve.
- **Sketch**: Create a temp file, build a `Devcontainer` with `remote_user = "root"`, call `copy_gitconfig` through `post_create`, assert `MockProvider::exec_calls` contains `mkdir -p -- '/root'` and `cp` call.

#### Uncovered: `Devcontainer::create_args` — `remote_env` and `run_args` paths (lines L247–L264)
- **Gap type**: Partial — the function itself is tested indirectly, but the non-empty `remote_env` and `run_args` branches are not covered.
- **Risk**: Low — incorrect args are passed to `docker create`, causing container boot failure at runtime rather than at test time.
- **Feasibility**: Pure unit test — no container needed.
- **Sketch**: Construct a `Config` with `remote_env = {"FOO": "bar"}` and `run_args = ["--privileged"]`; call `create_args()` and assert the returned vec contains `-e FOO=bar` and `--privileged`.

#### Uncovered: `exec_host_hook` (lines L41–L47)
- **Gap type**: No unit test
- **Risk**: Medium — used for `initializeCommand` which runs on the host. A bug here (e.g., failure to propagate non-zero exit) silently lets a failed pre-flight hook proceed.
- **Feasibility**: Unit test possible by passing a `OneOrMany::One("true")` / `"false"` shell command.
- **Sketch**: `exec_host_hook(&OneOrMany::One("true".into()))` → `Ok(true)`; `exec_host_hook(&OneOrMany::One("false".into()))` → `Ok(false)`; `exec_host_hook(&OneOrMany::Many(vec![]))` → `Ok(true)`.

#### Uncovered: `build_provider` Podman branches (lines L361–L391)
- **Gap type**: No unit test (settings-driven dispatch)
- **Risk**: Medium — The Podman path through `build_provider` constructs a different struct (`Podman` vs `Docker`). A missing field propagation (e.g., `forward_ports` not forwarded) would silently regress.
- **Feasibility**: Requires `Settings { provider: Provider::Podman, .. }` — no real container. Test can inspect the returned `Box<dyn Provider>` via downcast or by observing command construction.
- **Sketch**: Call `build_provider(dir, &Settings { provider: Provider::Podman, .. }, &config)` and assert `Ok`.

---

### `src/devcontainers/config.rs`

#### Uncovered: `should_shutdown` with `ShutdownAction::None` (line L114–L116)
- **Gap type**: No unit test for `None` variant
- **Risk**: Low — if `ShutdownAction::None` is parsed but treated as `StopContainer`, the container is stopped unexpectedly on exit. Annoying but not data-destroying.
- **Feasibility**: Unit test possible — parse a fixture containing `"shutdownAction": "none"` and assert `should_shutdown() == false`.
- **Sketch**: Add a fixture `devcontainer_shutdown_none.json` with `"shutdownAction": "none"`; assert `config.should_shutdown() == false`.

#### Uncovered: `is_compose` with `dockerComposeFile` absent (implicitly covered) / `build_args` with non-empty build (line L88–L90)
- **Gap type**: Partial
- **Risk**: Low
- **Feasibility**: Pure unit test via fixture parsing.
- **Sketch**: Parse a fixture with `build.args = {"KEY": "val"}` and assert `build_args()` returns the expected map.

---

### `src/provider/podman_compose.rs`

#### Uncovered: All `Provider` method bodies except `extract_container_id`, `running_args`, `rm_args` (lines L61–L298)
- **Gap type**: Integration only (skipped when `podman-compose` absent)
- **Risk**: High — `PodmanCompose` has the lowest line coverage (37.81%). The `cp()` method has a unique two-step flow (resolve container ID via label, then `podman cp`) that differs from all other providers. A regression here — e.g., the `container_id.is_empty()` early return never triggering — silently skips file copies.
- **Feasibility**: `build`, `start`, `stop`, `restart`, `attach`, `exec`, `exec_raw`, `rm` all shell out to `podman-compose` and cannot be unit-tested without the binary. However, `cp()` contains pure logic (the `extract_container_id` call and empty-string guard) that can be unit-tested with a mock or by testing `extract_container_id` directly (already done). The "container ID empty → return false" path is not tested end-to-end.
- **Sketch**: For the empty-container-id path in `cp()`, a unit test is not directly possible without mocking `Command`; recommend an integration test with a stopped compose project (where the service container is absent) asserting `cp()` returns `Ok(false)`.

---

### `src/provider/docker.rs`

#### Uncovered: `Docker::build` with `BuildSource::Image` (pull path) (lines L58–L65)
- **Gap type**: No unit test / integration test covers only Dockerfile path
- **Risk**: Medium — if the image-pull path omits a required flag or uses the wrong subcommand, pulling a pre-built image fails silently.
- **Feasibility**: Integration test — requires network access to pull `alpine:latest` or similar. Could also be tested by pointing `build_source` at a locally available image.
- **Sketch**: Construct a `Docker { build_source: BuildSource::Image("alpine:latest".into()), .. }` and assert `build(true)` succeeds.

#### Uncovered: `Docker::create` with mounts present (lines L103–L114)
- **Gap type**: No unit test
- **Risk**: Low-Medium — the mounts iterator builds `--mount key=val,...` strings; a formatting bug breaks volume mounts silently.
- **Feasibility**: Integration test — create a container with `mounts: Some(vec![{"type": "tmpfs", "target": "/data"}])` and verify the container starts.
- **Sketch**: Extend `test_docker_build_and_create` with a non-None `mounts` value.

#### Uncovered: `Docker::create` with `override_command = false` (line L124–L129)
- **Gap type**: No test for the `false` branch
- **Risk**: Low — most containers set `override_command = true`; the false branch is the spec-default but never tested.
- **Feasibility**: Integration test tweak — set `override_command: false` and assert `create` + `start` succeed.

---

### `src/provider/utils.rs`

#### Uncovered: `create_compose_override` with `SSH_AUTH_SOCK` absent (lines L42–L51 — else branch)
- **Gap type**: No dedicated unit test (line coverage is 86% because template rendering is exercised, but the SSH branch is conditional on env var)
- **Risk**: Low — when `SSH_AUTH_SOCK` is absent, the `volumes` vec stays empty and the template renders without the agent socket. A template rendering bug in that case would cause a panic in CI environments without an SSH agent.
- **Feasibility**: Unit test possible — temporarily unset `SSH_AUTH_SOCK` via `std::env::remove_var` (within a test with `#[serial]` or careful scoping) and call `create_compose_override("svc", &[])`, asserting the rendered YAML contains no `SSH_AUTH_SOCK` entries.
- **Sketch**: `temp_env::with_var_unset("SSH_AUTH_SOCK", || { let path = create_compose_override("svc", &[]).unwrap(); let contents = fs::read_to_string(path).unwrap(); assert!(!contents.contains("SSH_AUTH_SOCK")); })`.

---

### `src/commands/rebuild.rs` / `src/commands/start.rs`

#### Uncovered: `get_project_directory` — env-expansion path (lines L14–L26 both files)
- **Gap type**: No unit test
- **Risk**: Low — the `None` branch delegates to `std::env::current_dir()`; the `Some(path)` branch does env expansion via `shellexpand::env` followed by `canonicalize()`. A bad env var reference would produce an opaque error with no user-friendly message wrapping.
- **Feasibility**: Unit test possible without a container — pass `Some("$HOME")` and assert `Ok`.
- **Sketch**: `get_project_directory(Some("$HOME"))` → `Ok(...)` and value equals `std::env::var("HOME").unwrap()`.

---

### `src/settings.rs`

#### Uncovered: `Settings::try_load` — file-present path (lines L53–L62)
- **Gap type**: No unit test (only "no config file" case is tested)
- **Risk**: Low — a TOML parse error produces a warning and falls back to defaults. If the fallback silently swallows a real misconfiguration, the user gets Docker when they intended Podman.
- **Feasibility**: Unit test possible — write a temp TOML file with `provider = "podman"`, set `XDG_CONFIG_HOME` to the temp dir, call `Settings::load()`, assert provider is `Podman`.
- **Sketch**: Use `tempfile::tempdir()`, write the TOML, set env var, call `Settings::load()`, assert.

---

## 3. Priority Matrix

| Priority | File | Function | Risk | Feasibility | Action |
|---|---|---|---|---|---|
| P1 | `devcontainers/mod.rs` | `Devcontainer::run` | High | Unit test possible | Add MockProvider-based lifecycle sequence tests |
| P1 | `devcontainers/mod.rs` | `exec_host_hook` | Medium | Unit test possible | Add shell-command round-trip tests |
| P1 | `provider/podman_compose.rs` | All `Provider` impls | High | Integration test | Add integration tests for Podman Compose path |
| P2 | `devcontainers/mod.rs` | `Devcontainer::rebuild` | Medium | Unit test possible | Test exists-true and exists-false branches |
| P2 | `devcontainers/mod.rs` | `Devcontainer::post_create` | Medium | Unit test possible | Test each lifecycle hook variant |
| P2 | `devcontainers/mod.rs` | `Devcontainer::copy` / `copy_gitconfig` / `copy_dotfiles` | Medium | Unit test possible (path-quoting) | Test quoting, root vs non-root home dir |
| P2 | `provider/docker.rs` | `Docker::build` Image path | Medium | Integration test | Add `BuildSource::Image` test |
| P3 | `devcontainers/config.rs` | `should_shutdown` None variant | Low | Unit test via fixture | Add fixture + assertion |
| P3 | `provider/utils.rs` | `create_compose_override` no SSH agent | Low | Unit test with env var unset | Add env-scoped test |
| P3 | `commands/start.rs`, `commands/rebuild.rs` | `get_project_directory` | Low | Unit test | Test env expansion path |
| P3 | `settings.rs` | `Settings::try_load` file-present | Low | Unit test with temp file | Write TOML to temp dir and assert |
| P4 | `provider/docker.rs` | `create` with mounts | Low | Integration test | Extend existing create test |
| P4 | `devcontainers/mod.rs` | `create_args` env/run_args | Low | Pure unit test | Test non-empty remote_env and run_args |

---

## 4. Follow-up Track Stubs

1. **`test_gaps_devcontainer_run_20260309`** — Unit tests for `Devcontainer::run()` and `Devcontainer::rebuild()` lifecycle using the existing `MockProvider`. Covers: happy path, `initializeCommand` failure aborts run, `should_shutdown()` calls stop, container already running skips start, container already exists skips build/create.

2. **`test_gaps_post_create_hooks_20260309`** — Unit tests for `Devcontainer::post_create()`, `copy()`, `copy_gitconfig()`, `copy_dotfiles()`. Covers: all three lifecycle hooks (Some/None), path quoting for special characters, root vs non-root home directory resolution, missing dotfile source returns error.

3. **`test_gaps_exec_host_hook_20260309`** — Unit tests for `exec_host_hook()`. Covers: `One` string form exits 0 → `Ok(true)`, exits non-zero → `Ok(false)`, empty `Many` → `Ok(true)`, non-empty `Many` invokes program directly.

4. **`test_gaps_podman_compose_provider_20260309`** — Integration tests for `PodmanCompose` build/start/stop/restart/exec/cp/rm paths that are currently skipped when `podman-compose` is absent. Add a dedicated CI job or fixture that ensures these paths are exercised. Also add a unit test for `cp()` when the container ID is empty (returns `Ok(false)`).

5. **`test_gaps_docker_image_source_20260309`** — Integration test for `Docker::build` with `BuildSource::Image` (pull path) and `Docker::create` with non-None `mounts`.

6. **`test_gaps_config_shutdown_and_build_args_20260309`** — Unit tests for `Config::should_shutdown()` with `ShutdownAction::None`, and `Config::build_args()` with a non-empty `args` map.

7. **`test_gaps_settings_and_commands_20260309`** — Unit tests for `Settings::try_load()` with a TOML file on disk, and for `commands::start::get_project_directory` / `commands::rebuild::get_project_directory` with env-variable paths.

8. **`test_gaps_utils_no_ssh_agent_20260309`** — Unit test for `create_compose_override` when `SSH_AUTH_SOCK` is not set, asserting the rendered override YAML omits the socket volume and env entry.
