# Spec: PodmanCompose Unit Tests — running(), cp(), rm() Bug Fix Coverage

## Problem

Three bugs were fixed in the podman-compose Provider Completion track:
- `running()` — was using `--format {{.ID}}` which conflicts with `--filter`; fixed to `--filter status=running`
- `cp()` — container ID resolution took entire stdout including trailing newlines; fixed to take first non-empty line
- `rm()` — `--rmi all` removed orphaned images but caused failures; removed

These fixes have integration test coverage but no targeted unit tests. If the fixes regress, the
integration tests (which require a running Podman daemon) would catch it, but that's slow feedback.

## Goal

Add unit tests for each of the three fixed behaviors in `src/provider/podman_compose.rs`.
Use command-argument inspection rather than running real containers (mock or inspect the built
`Command` before execution).

## Functional Requirements

- FR-001: Unit test verifying `running()` command includes `--filter status=running` and does NOT include `--format`
- FR-002: Unit test verifying `cp()` correctly picks first non-empty line from multi-line output
- FR-003: Unit test verifying `rm()` command does NOT include `--rmi` or `all`

## Approach

For FR-001 and FR-003, extract command construction into a testable helper that returns
`Command` (or a Vec of args) before execution, so tests can inspect args without running
the command.

For FR-002, the ID extraction logic can be unit-tested as a pure string-processing function.
