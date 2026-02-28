# Spec: Code Inventory

## Overview

Audit the `devcont` codebase for code that is dead, duplicated, or misaligned with
the product's stated goals. For each identified item, produce a written decision
(Remove / Refactor / Retain with justification), then implement all approved changes
and verify the quality gate.

The audit is scoped to `src/` only. Test code, fixtures, and Conductor files are
out of scope.

**Depends on:** none (standalone chore)

---

## Pre-identified Items

The following items were identified during codebase review and must be addressed.
Additional items discovered during the audit should be added to `findings.md`.

### Item 1 — `Error::Provider` (src/error.rs:17)

`Error::Provider(String)` carries `#[allow(dead_code)]`. It is never constructed
in the codebase. All provider errors are currently surfaced via `std::io::Error`
directly. Decision must be: Remove (simplify the error type) or Retain with a
concrete plan and timeline for use.

### Item 2 — `Config::file` (src/devcontainers/config.rs:27)

`Config::file: Option<String>` carries `#[allow(dead_code)]` with the comment
"Retained for devcontainer spec completeness; may be used in future features."
The Dev Containers spec does not mandate this field on the Rust side — it is a
parsed JSON field that has no effect. Decision: Remove (reduce noise) or Retain
with a specific spec citation and a linked future track.

### Item 3 — `Build::context` (src/devcontainers/config.rs:58)

`Build::context: Option<String>` carries `#[allow(dead_code)]`. The build context
is always set to `directory` (the project root) by all providers, ignoring this
field. Decision: Remove or wire it up and remove the `allow`.

### Item 4 — `ShutdownAction::StopCompose` (src/devcontainers/config.rs:11)

The `ShutdownAction` enum has three variants (`None`, `StopContainer`,
`StopCompose`), but `should_shutdown()` returns `!matches!(self.shutdown_action,
ShutdownAction::None)` — treating `StopCompose` identically to `StopContainer`.
There is no code path that distinguishes them. Decision: Implement the distinction
(stop only compose services), collapse to a bool, or document why the current
simplification is acceptable.

### Item 5 — `src/provider/utils.rs` (1 line, empty)

The file declares nothing — it is a placeholder. Decision: Delete it and remove
the implicit `mod utils` entry (if any), or populate it with extracted shared logic.

### Item 6 — Unused fields on `DockerCompose` and `PodmanCompose`

Both compose structs suppress dead_code with `#[allow(dead_code)]` on the struct.
The following fields are stored but never read by any method:

| Struct | Field | Stored by | Used by |
|--------|-------|-----------|---------|
| `DockerCompose` | `directory` | `build_provider` | — |
| `DockerCompose` | `forward_ports` | `build_provider` | — |
| `DockerCompose` | `run_args` | `build_provider` | — |
| `PodmanCompose` | `directory` | `build_provider` | — |
| `PodmanCompose` | `forward_ports` | `build_provider` | — |
| `PodmanCompose` | `run_args` | `build_provider` | — |

Decision: Remove the fields (and update `build_provider`) or implement them
(e.g., pass `run_args` through to the compose up command).

### Item 7 — Duplicated `create_docker_compose()` method

`DockerCompose::create_docker_compose()` and `PodmanCompose::create_docker_compose()`
are functionally identical private methods (same template, same logic, same
output path). Decision: Extract to a shared free function in `provider/utils.rs`
(or a new shared module) or document why the duplication is preferable.

---

## Functional Requirements

### FR-001: Findings document

Produce `conductor/tracks/code_inventory_20260228/findings.md` listing every
identified item with:
- Location (`file:line`)
- Description
- Decision: **Remove** | **Refactor** | **Retain** (with mandatory justification
  for Retain)

All 7 pre-identified items must appear. Any additional items found during the audit
must also be included.

### FR-002: Implement all Remove and Refactor decisions

Each item marked Remove or Refactor must be implemented in a separate commit.
Items marked Retain need only the justification in `findings.md`.

### FR-003: No new `#[allow(dead_code)]` suppressions

After implementation, the codebase must not introduce any new `#[allow(dead_code)]`
annotations. Existing Retain items must carry an inline comment referencing the
findings document.

### FR-004: Quality gate passes

`cargo test`, `cargo test --test integration`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt --check`, and `cargo deny check` must all pass after implementation.

---

## Non-Functional Requirements

- Changes are minimal and surgical — no opportunistic refactoring beyond what
  the findings prescribe
- Each Remove/Refactor is a separate commit (one concern per commit)

---

## Acceptance Criteria

- `findings.md` exists and covers all 7 pre-identified items plus any discovered extras
- All Remove decisions are implemented
- All Refactor decisions are implemented
- No `#[allow(dead_code)]` silences an item without a Retain justification in findings.md
- Quality gate passes

---

## Out of Scope

- New features
- Architecture rework
- Adding tests for currently-untested code paths
- Anything in `tests/`, `conductor/`, or config files
