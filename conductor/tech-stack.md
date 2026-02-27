# Tech Stack: devcont

## Language

- **Rust** — edition 2024, MSRV 1.85+

## CLI

- **clap 4.x** (derive API) — argument parsing and subcommand routing
  - Replaces clap 3.x currently in use

## Error Handling

- **anyhow** — application-level error propagation with context
- **thiserror** — typed error definitions for library-facing code
- Replaces all bare `unwrap()` / `expect()` on user-facing code paths

## Serialization

- **serde** + **serde_json** — JSON serialization
- **json5** — parsing `devcontainer.json` (supports comments and trailing commas)
- **toml** — parsing user config (`~/.config/devcont/config.toml`)

## Filesystem & OS

- **directories** — OS-appropriate config/data directory resolution
- **shellexpand** — `~` and `$VAR` expansion in paths

## Templating

- **tinytemplate** — docker-compose.yml template rendering

## Testing

- Rust built-in test framework (`#[test]`, `#[cfg(test)]`)
- **cargo-llvm-cov** — code coverage measurement (target: >80%)
- **cargo-fuzz** (libFuzzer) — fuzz testing for config parsing entry points

## CI/CD

- **GitHub Actions** — build, lint (`clippy`), test, and release workflows
- **cargo-deny** — dependency audit (licenses, advisories)

## Container Engines Supported

- Docker
- Podman
- docker-compose
- podman-compose (planned completion)

## Dev Container

- Base image: official `rust` Docker image (latest stable, e.g. `rust:1-slim-bookworm`)
- Updated from the legacy mcr.microsoft.com/vscode/devcontainers/rust image
