# Spec: Refactor — Tighten Provider Module Visibility

## Problem

Several functions and types in `src/provider/` are marked `pub` but are only used within the
crate or within the provider module. Overly broad visibility enlarges the public API surface,
makes future refactoring harder, and can cause confusion about what is intentionally public.

Key findings from the code review:
- `print_command` in `src/provider/mod.rs` is `pub` but only used internally.
- Helper functions in individual provider files (`extract_container_id`, `running_args`, etc.)
  are `pub` when they should be `pub(crate)` or private.
- `BuildSource` variants are public but never constructed outside the crate.

## Goal

Reduce every public item to its minimum required visibility.

## Functional Requirements

- FR-001: Audit every `pub` item in `src/provider/` and `src/devcontainers/`.
- FR-002: Change `pub` → `pub(crate)` for items used only within the crate but across modules.
- FR-003: Change `pub(crate)` → private for items used only within their own module.
- FR-004: `cargo build` must pass after each change.
- FR-005: `cargo clippy` must pass (dead_code warnings resolved).
- FR-006: No behavioural changes.

## Out of Scope

- Adding or removing public API items.
- Documenting public items (tracked in `rustdoc_gaps`).
