# Spec: Security — Validate runArgs Against Allowlist

## Problem

`runArgs` in `devcontainer.json` is passed verbatim to the container runtime (`docker run` or
`podman run`) without any validation. A malicious or compromised `devcontainer.json` in a cloned
repository can inject arbitrary flags such as `--privileged`, `--cap-add=ALL`, `--security-opt
seccomp=unconfined`, or dangerous volume mounts (`-v /:/host`), silently escalating the container
to root on the host.

STRIDE classification: **Elevation of Privilege** (Critical).

## Goal

Validate `runArgs` before they are passed to the runtime, and reject or strip flags that could
escalate container privileges or break isolation.

## Functional Requirements

- FR-001: Define an allowlist of safe `runArgs` prefixes/flags (e.g., `--env`, `--env-file`,
  `--label`, `--name`, `--hostname`, `--network`, `--expose`, `--publish`, `--dns`, `--add-host`,
  `--workdir`, `--user`, `--memory`, `--cpus`).
- FR-002: Define a denylist of privilege-escalating flags (e.g., `--privileged`, `--cap-add`,
  `--cap-drop`, `--security-opt`, `--device`, `--pid`, `--ipc`, `--userns`, `--cgroupns`).
- FR-003: At container creation time, validate all `runArgs` entries against the allowlist/denylist.
- FR-004: If a deny-listed flag is present, abort with a clear error message naming the flag.
- FR-005: If a flag is not in the allowlist, emit a warning (not a hard abort) to allow
  forward-compatibility with new runtime flags, but log it prominently.
- FR-006: The allowlist is defined as a constant in `src/devcontainers/run_args.rs` (new file).
- FR-007: Unit tests cover: allowed flags pass, denied flags abort, unknown flags warn, empty list ok.

## Out of Scope

- Filtering `--volume` / `-v` mounts (tracked separately as `sec_path_traversal`).
- Filtering `--security-opt seccomp=...` beyond simple name match (future).
