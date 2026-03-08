# Code Inventory Findings

Audit of `src/` — dead, duplicated, or misaligned code.
Scope: `src/` only. Test files, fixtures, and Conductor files are out of scope.
Date: 2026-02-28.

---

## Item 1 — `Error::Provider` (`src/error.rs:16–17`)

**Location:** `src/error.rs`, lines 16–17
**Description:** `Error::Provider(String)` variant carries `#[allow(dead_code)]`. It is never
constructed anywhere in the codebase. All provider errors are surfaced via `std::io::Error`
directly (providers return `std::io::Result`).
**Decision: Remove**
Rationale: The variant has no callsite and no concrete plan for use. Retaining it adds noise
to the error type and silences a legitimate compiler warning. If a structured provider error
type is needed in the future, a new variant should be introduced with purpose at that time.

---

## Item 2 — `Config::file` (`src/devcontainers/config.rs:26–27`)

**Location:** `src/devcontainers/config.rs`, lines 26–27
**Description:** `Config::file: Option<String>` carries `#[allow(dead_code)]` with the comment
"Retained for devcontainer spec completeness; may be used in future features." This field is
not part of the top-level Dev Containers spec schema (`devcontainer.json` does not have a
root-level `file` key separate from `dockerComposeFile`). It has no callsite.
**Decision: Remove**
Rationale: The field is not mandated by the spec at this level, has no callsite, and the
existing comment provides no specific spec citation or linked future track. It produces
noise without value.

---

## Item 3 — `Build::context` (`src/devcontainers/config.rs:57–58`)

**Location:** `src/devcontainers/config.rs`, lines 57–58
**Description:** `Build::context: Option<String>` carries `#[allow(dead_code)]`. The Dev
Containers spec explicitly defines `build.context` as "Path that the Docker build should be
run from relative to devcontainer.json." All providers currently pass the project directory
as build context unconditionally, ignoring this field.
**Decision: Retain**
Rationale: `build.context` is a first-class field in the Dev Containers spec. It is already
parsed from config files; removing it would silently discard user-supplied values. A future
track should implement wiring of this field into the provider build commands.
Action: Replace `#[allow(dead_code)]` with an inline comment referencing this findings entry.

---

## Item 4 — `ShutdownAction::StopCompose` (`src/devcontainers/config.rs:11`)

**Location:** `src/devcontainers/config.rs`, lines 8–12 (enum) and line 107 (`should_shutdown`)
**Description:** The `ShutdownAction` enum has three variants (`None`, `StopContainer`,
`StopCompose`), but `should_shutdown()` returns `!matches!(self.shutdown_action, ShutdownAction::None)`,
treating `StopCompose` identically to `StopContainer`. No code path distinguishes the two.
**Decision: Retain**
Rationale: The Dev Containers spec defines `stopCompose` as a valid `shutdownAction`. Collapsing
to a `bool` would lose the ability to parse this value correctly from config files. The current
behaviour (stop-on-exit regardless of variant) is a conservative simplification that is safe: it
never leaves a compose project running unintentionally. A future track should implement the
distinction — stop only the service container rather than the entire project.
Action: Add an inline comment in `should_shutdown()` explaining the current simplification.

---

## Item 5 — `src/provider/utils.rs` (empty file)

**Location:** `src/provider/utils.rs`
**Description:** The file is completely empty (0 bytes). It declares no items. It is referenced
as `pub mod utils;` (or `mod utils;`) in `src/provider/mod.rs`.
**Decision: Remove**
Rationale: An empty file contributes nothing and creates the false impression that a utility
module exists. If shared utilities are needed in the future (e.g., a shared `create_compose_override`
function from Item 7), the file should be recreated with actual content at that time.

---

## Item 6 — Unused fields on `DockerCompose` and `PodmanCompose`

**Location:**
- `src/provider/docker_compose.rs:12–24` — struct `DockerCompose`
- `src/provider/podman_compose.rs:12–25` — struct `PodmanCompose`

**Description:** Both structs carry a struct-level `#[allow(dead_code)]`. The following fields
are populated at construction but never read by any method in the `impl` blocks:

| Struct | Field | Type | Stored | Read |
|--------|-------|------|--------|------|
| `DockerCompose` | `directory` | `String` | yes | no |
| `DockerCompose` | `forward_ports` | `Vec<u16>` | yes | no |
| `DockerCompose` | `run_args` | `Vec<String>` | yes | no |
| `PodmanCompose` | `directory` | `String` | yes | no |
| `PodmanCompose` | `forward_ports` | `Vec<u16>` | yes | no |
| `PodmanCompose` | `run_args` | `Vec<String>` | yes | no |

**Decision: Remove**
Rationale:
- `directory`: The compose providers use `self.file` (the compose file path) and the override
  compose file to drive all commands. The working directory is not passed to any command and
  is not needed for the current implementation.
- `forward_ports` and `run_args`: These are legitimate devcontainer spec fields but are not
  implemented in the compose providers. Storing them silently without acting on them is
  misleading. A dedicated future track should implement port forwarding and run-args for
  compose providers when the need arises.
Action: Remove the three fields from both structs, remove the struct-level `#[allow(dead_code)]`,
and update the construction calls in `src/devcontainers/mod.rs`.

---

## Item 7 — Duplicated `create_docker_compose()` method

**Location:**
- `src/provider/docker_compose.rs:41–75` — `DockerCompose::create_docker_compose()`
- `src/provider/podman_compose.rs:42–76` — `PodmanCompose::create_docker_compose()`

**Description:** Both methods are functionally identical: same SSH-agent forwarding logic,
same template (`templates/docker-compose.yml`), same output path (`env::temp_dir()/docker-compose.yml`),
and same return type. The only variation is `self.service` which is accessed from the struct
in both.

**Decision: Refactor**
Rationale: The duplication violates DRY and means any future change (e.g., different temp
file name, different SSH forwarding logic) must be applied twice. Extract to a free function
`create_compose_override(service: &str) -> std::io::Result<String>` in a new or restored
`src/provider/utils.rs` and replace both methods with calls to it.

---

## Additional Items Discovered

No additional items were found during the audit. The five `#[allow(dead_code)]` annotations
in `src/` (across four files) correspond exactly to Items 1–6 above.

---

## Summary

| Item | Location | Decision |
|------|----------|----------|
| 1 | `src/error.rs:16–17` | **Remove** |
| 2 | `src/devcontainers/config.rs:26–27` | **Remove** |
| 3 | `src/devcontainers/config.rs:57–58` | **Retain** (add spec comment) |
| 4 | `src/devcontainers/config.rs:8–12, 107` | **Retain** (add simplification comment) |
| 5 | `src/provider/utils.rs` | **Remove** |
| 6 | `DockerCompose`, `PodmanCompose` unused fields | **Remove** |
| 7 | `create_docker_compose()` duplication | **Refactor** |
