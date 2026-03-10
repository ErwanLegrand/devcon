# Spec: Fix — Podman Providers Missing mounts and remoteEnv Support

## Problem

The Docker and DockerCompose providers correctly pass `mounts` and `remoteEnv` from
`devcontainer.json` to the container runtime. The Podman and PodmanCompose providers have
incomplete implementations — `mounts` entries are not appended to the `podman run` / compose
override, and `remoteEnv` entries are not injected into the compose override template.

This causes a feature parity gap: the same `devcontainer.json` works with Docker but silently
ignores mounts/env with Podman.

## Goal

Implement full `mounts` and `remoteEnv` support in `Podman` and `PodmanCompose` providers,
matching Docker's behaviour.

## Functional Requirements

- FR-001: `Podman::create()` appends all `mounts` entries as `--mount` or `-v` arguments.
- FR-002: `PodmanCompose::create()` injects `mounts` into the compose override YAML.
- FR-003: `PodmanCompose` compose override template includes `remoteEnv` entries as environment
  variables in the service definition.
- FR-004: Parity test: the same mount and env config produces equivalent runtime commands for
  Docker and Podman providers.
- FR-005: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Validating mount source paths (tracked in `sec_path_traversal`).
- SELinux `:z` relabelling (tracked in `sec_podman_selinux`).
