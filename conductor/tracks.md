# Project Tracks

This file tracks all major tracks for the project. Each track has its own detailed plan in its respective folder.

---

## [x] Track: Foundation Modernization
*Rename to `devcont`, Rust 2024 + MSRV 1.85, dependency upgrades, official rust dev container, rust-dev-template integration, CI quality gates.*
*Link: [./tracks/foundation_modernization_20260227/](./tracks/foundation_modernization_20260227/)*

---

## [ ] Track: Code Inventory
*Audit the codebase for code unnecessary to stated goals; document findings and decide on removal or refactoring.*
*(To be planned)*

---

---

## [ ] Track: Docker-outside-of-Docker for Dev Container
*Add DooD support (socket mount + Docker CLI) to the dev container, and a minimal integration test suite (`tests/integration.rs`) with purpose-built fixtures.*
*Link: [./tracks/dood_devcontainer_20260228/](./tracks/dood_devcontainer_20260228/)*

---

## Future Tracks (not yet planned)

- Dev Containers spec compliance audit and gap fixes
- Architecture improvements (provider abstraction, error handling patterns, module structure)
- Thorough testing and fuzzing (unit, integration, fuzz harnesses for config parsing)
- `cargo devcont` subcommand packaging and distribution
- podman-compose provider completion
- Documentation (man page, improved README, inline doc coverage)
