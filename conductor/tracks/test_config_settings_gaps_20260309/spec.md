# Spec: Tests — Config and Settings — Full Field Coverage and Deserialization

## Problem

`src/devcontainers/config.rs` and `src/settings.rs` have low test coverage:
- All 17 `devcontainer.json` fields need deserialization tests.
- Only `image`, `build`, and a couple of hook fields are covered by existing fixtures.
- Missing fields: `mounts`, `remoteEnv`, `containerEnv`, `shutdownAction`, `overrideCommand`,
  `forwardPorts`, `remoteUser`, `containerUser`, `runArgs`, `selinuxRelabel`.
- Settings deserialization and default value tests are minimal.

Baseline coverage: `config.rs` ~60%, `settings.rs` ~40%.

## Goal

Achieve ≥85% line coverage for `config.rs` and ≥80% for `settings.rs` through comprehensive
deserialization tests.

## Functional Requirements

- FR-001: One test per `devcontainer.json` field verifying: field present → parsed correctly,
  field absent → default value applied.
- FR-002: Test `mounts` as array of strings.
- FR-003: Test `remoteEnv` and `containerEnv` as maps.
- FR-004: Test `shutdownAction` variants: `"none"`, `"stopContainer"`, `"stopCompose"`.
- FR-005: Test `overrideCommand: false`.
- FR-006: Test `forwardPorts` as array.
- FR-007: Test `runArgs` as array.
- FR-008: Test all six lifecycle hooks in both string and array form.
- FR-009: Settings tests: all engine variants round-trip, unknown engine returns error.
- FR-010: `cargo test` must pass. Coverage targets met.

## Out of Scope

- Testing invalid field types (JSON type mismatch) — covered by `test_exec_host_hook_and_utils`.
