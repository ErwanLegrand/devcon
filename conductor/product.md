# Product Guide: devcont

## Vision

`devcont` is a cross-platform CLI tool that implements the [Dev Containers specification](https://containers.dev/)
outside of Visual Studio Code. It enables developers to start, rebuild, and manage dev containers from any
terminal, with full support for Docker, Podman, docker-compose, and podman-compose.

The tool is distributed as:
- `devcont` — a standalone binary
- `cargo devcont` — a Cargo subcommand for Rust projects

## Target Users

- Developers who use devcontainers but prefer non-VS Code editors (Neovim, Emacs, JetBrains IDEs, etc.)
- Teams enforcing reproducible development environments via the Dev Containers spec
- CI/CD pipelines that need to spin up devcontainers programmatically
- Rust developers who want a first-class `cargo` integration

## Goals

1. **Spec compliance**: Implement the Dev Containers specification faithfully; document and fix any discrepancies
2. **Dual distribution**: Ship as both a standalone `devcont` binary and a `cargo devcont` subcommand
3. **Modern Rust**: Use Rust edition 2024, targeting stable 1.85+
4. **Architecture quality**: Clean module boundaries, testable code, proper error handling via `thiserror`/`anyhow`
5. **Security**: Validate all inputs, avoid command injection, handle secrets safely
6. **Thorough testing**: Unit, integration, and fuzz tests with >80% coverage
7. **Documentation**: Comprehensive inline docs, user-facing README, and man page

## Features

### Implemented
- `devcont start [dir]` — starts the dev container in the given directory
- `devcont rebuild [--no-cache] [dir]` — rebuilds and starts the container
- Docker and Podman provider support
- docker-compose support
- SSH agent forwarding
- dotfiles support

### Planned
- Full spec compliance audit and gap fixes
- podman-compose provider (currently incomplete)
- `cargo devcont` subcommand packaging
- Dev container lifecycle hooks (`onCreateCommand`, `postCreateCommand`, etc.)
- Port forwarding improvements
- Mount handling improvements
- Fuzzing harnesses for config parsing

## Non-Goals

- VS Code extension / GUI — this is a CLI-only tool
- Managing container registries
- Building/pushing images beyond what the spec requires

## Success Criteria

- All Dev Containers spec features tested against the official spec test suite
- `cargo devcont` works as a first-class Cargo subcommand
- Zero `unwrap()` panics on user-facing code paths
- >80% test coverage across all modules
- Clean `clippy` with no warnings
- Fuzz tests for all config parsing entry points
