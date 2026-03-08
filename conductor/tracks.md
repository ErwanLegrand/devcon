# Project Tracks

This file tracks all major tracks for the project. Each track has its own detailed plan in its respective folder.

---

## [x] Track: Foundation Modernization
*Rename to `devcont`, Rust 2024 + MSRV 1.85, dependency upgrades, official rust dev container, rust-dev-template integration, CI quality gates.*
*Link: [./archive/foundation_modernization_20260227/](./archive/foundation_modernization_20260227/)*

---

---

## [x] Track: Docker-outside-of-Docker for Dev Container
*Add DooD support (socket mount + Docker CLI) to the dev container, and a minimal integration test suite (`tests/integration.rs`) with purpose-built fixtures.*
*Link: [./archive/dood_devcontainer_20260228/](./archive/dood_devcontainer_20260228/)*

---

## [x] Track: Docker Provider Integration Tests
*Comprehensive integration tests for `Docker` and `DockerCompose` providers — all `Provider` trait methods. Blocked on: DooD Dev Container track.*
*Link: [./archive/integration_tests_docker_20260228/](./archive/integration_tests_docker_20260228/)*

---

## [x] Track: Podman Provider Integration Tests
*Comprehensive integration tests for `Podman` and `PodmanCompose` providers — all `Provider` trait methods. Blocked on: DooD Dev Container track + Docker integration tests track.*
*Link: [./archive/integration_tests_podman_20260228/](./archive/integration_tests_podman_20260228/)*

---

## Future Tracks (not yet planned)

- Dev Containers spec compliance audit and gap fixes
- Architecture improvements (provider abstraction, error handling patterns, module structure)
- Thorough testing and fuzzing (unit, integration, fuzz harnesses for config parsing)
- `cargo devcont` subcommand packaging and distribution
- podman-compose provider completion
- Documentation (man page, improved README, inline doc coverage)
