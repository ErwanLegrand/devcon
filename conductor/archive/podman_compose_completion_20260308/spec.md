# Spec: podman-compose Provider Completion

## Overview

Fix concrete bugs and missing wiring in the `PodmanCompose` provider identified
during the Code Inventory audit and a subsequent runtime gap analysis. The fixes
also apply to `DockerCompose` where the same bugs exist.

**Depends on:** Code Inventory track (complete)

---

## Pre-identified Issues

### Issue 1 — `running()` has extraneous `--format` flag and missing status filter
`PodmanCompose::running()` passes both `-q` and `--format {{.ID}}` to `podman ps`.
These conflict; `-q` already restricts output to IDs. Additionally, unlike
`DockerCompose::running()`, there is no `--status=running` filter, making the
intent ambiguous across Podman versions.

### Issue 2 — `attach()` hardcodes `zsh` in both compose providers
`DockerCompose::attach()` and `PodmanCompose::attach()` hardcode `zsh` as the
interactive shell. Any image without `zsh` (e.g., Alpine) will fail to attach.

### Issue 3 — `cp()` does not handle scaled services
`PodmanCompose::cp()` passes the entire trimmed output of `podman ps` as the
container ID. If the project has multiple replicas, this becomes a multi-line
string and `podman cp` fails.

### Issue 4 — `remote_env` silently dropped in compose mode
`Devcontainer::create_args()` builds `-e KEY=VAL` flags for `remote_env`, but
both compose providers' `create()` method ignores its `args` parameter entirely
(`Ok(true)` stub). Environment variables from devcontainer.json have no effect
in compose mode.

### Issue 5 — `rm()` passes `--rmi all` which may not be supported
`PodmanCompose::rm()` passes `--rmi all` to `podman-compose down`. Older
versions of `podman-compose` do not support this flag, causing `rm()` to fail.

---

## Functional Requirements

### FR-001: Fix `running()` flag conflict
Remove `--format {{.ID}}` from `PodmanCompose::running()`. Add
`--filter status=running` to make the running-state check explicit and consistent
with `DockerCompose::running()`.

### FR-002: Make attach shell configurable with `sh` default
Add a `shell: String` field to both `DockerCompose` and `PodmanCompose` structs,
populated from user settings or a hardcoded default of `"sh"`. Use `self.shell`
in `attach()` instead of the hardcoded `"zsh"`. Update `build_provider` to pass
the shell value. Update integration test fixtures accordingly.

### FR-003: Fix `cp()` container ID resolution for scaled services
In `PodmanCompose::cp()`, take only the first non-empty line of `podman ps`
output as the container ID, rather than treating the full trimmed output as one
ID. Return `Ok(false)` if no matching container is found.

### FR-004: Inject `remote_env` via compose override template
Extend `TemplateContext` in `src/provider/utils.rs` to carry an `env_vars:
Vec<TemplateEntry>` field (key-value pairs from `remote_env`). Update
`create_compose_override` to accept `env_vars` and render them into the compose
override file under the service's `environment:` key. Update both `DockerCompose`
and `PodmanCompose` structs to carry `env_vars` and pass them at construction in
`build_provider`.

### FR-005: Remove `--rmi all` from `PodmanCompose::rm()`
Remove `--rmi all` from `PodmanCompose::rm()` to match integration test teardown
behaviour. This avoids failures on older `podman-compose` versions.

### FR-006: Integration test coverage for compose attach shell
Add an integration test that verifies `exec()` works with `sh` in both
`DockerCompose` and `PodmanCompose` fixtures (exercising the same code path as
`attach()` short of opening an interactive TTY).

---

## Non-Functional Requirements

- Each FR is a separate commit
- `cargo test --test integration` must pass after each change
- `cargo clippy --all-targets -- -D warnings` must pass

---

## Acceptance Criteria

- `PodmanCompose::running()` uses only `-q --filter status=running`
- `DockerCompose::attach()` and `PodmanCompose::attach()` use `self.shell` (default `"sh"`)
- `PodmanCompose::cp()` takes only the first container ID line
- `remote_env` entries appear in the compose override under `environment:`
- `PodmanCompose::rm()` does not pass `--rmi all`
- An integration test covers the exec/attach shell path for compose providers
- All quality gates pass

---

## Out of Scope

- `forward_ports` and `run_args` in compose providers
- `mounts` support in compose providers
- `override_command` support in compose providers
- `build.context` wiring
