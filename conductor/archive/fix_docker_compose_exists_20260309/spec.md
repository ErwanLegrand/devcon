# Spec: Fix — DockerCompose exists() Uses Wrong Container Name Source

## Problem

`DockerCompose::exists()` derives the expected container name differently from how Docker Compose
actually names containers (which uses `<project>_<service>_<index>` or
`<project>-<service>-<index>` depending on Compose version). The current implementation may
produce false negatives (container exists but `exists()` returns false), causing `start` to
re-create a running container.

The same logic bug likely affects `PodmanCompose::exists()`.

## Goal

Fix `exists()` for both compose providers to correctly detect whether the compose service
container exists, using the same name derivation that the runtime uses.

## Functional Requirements

- FR-001: Use `docker compose ps --format json <service>` (or equivalent) to list containers
  associated with the compose project, rather than guessing the name.
- FR-002: Apply the same fix to `PodmanCompose::exists()`.
- FR-003: Unit tests: parsing of compose ps output to extract container names, exists returns
  true when container is listed, false when not.
- FR-004: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Fixing `running()` (shares similar logic but is a separate concern).
