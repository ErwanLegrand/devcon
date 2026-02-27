# Spec: Foundation Modernization

## Overview

This track establishes the technical foundation for the `devcont` project. It covers re-branding,
language/toolchain modernization, dependency upgrades, dev container update, integration of the
`rust-dev-template` repository features, and CI quality gate enforcement.

All subsequent tracks (spec compliance, architecture improvements, testing, fuzzing) depend on this
track being complete.

## Goals

1. Rename the binary and project from `devcon` to `devcont`
2. Migrate to Rust edition 2024 with MSRV 1.85+
3. Replace outdated dependencies with modern equivalents
4. Replace all `unwrap()`/`expect()` on user-facing paths with proper error handling
5. Update the dev container to use the official `rust` Docker image
6. Integrate relevant features from `git@github.com:ErwanLegrand/rust-dev-template.git`
7. Enforce CI quality gates: clippy, rustfmt, cargo-deny, cargo-llvm-cov

## Requirements

### RE-001: Re-branding

- `Cargo.toml`: package name changed to `devcont`
- Binary name: `devcont` (via `[[bin]]` section)
- Container name prefix changed from `devcon-` to `devcont-`
- README updated to reflect new name and commands
- All internal references updated

### RE-002: Rust Edition 2024

- `Cargo.toml`: `edition = "2024"`, `rust-version = "1.85"`
- Code compiles cleanly under edition 2024 semantics
- No `unsafe` blocks introduced without justification

### RE-003: Dependency Upgrades

| Crate | Current | Target | Notes |
|-------|---------|--------|-------|
| clap | 3.1.18 | 4.x | Use derive feature; update CLI definitions |
| anyhow | — | 1.x | Add for application error handling |
| thiserror | — | 2.x | Add for typed domain errors |
| serde | 1.x | 1.x (latest) | Keep, update version pin |
| serde_json | 1.x | 1.x (latest) | Keep |
| json5 | 0.4.x | latest | Keep or replace with `serde_json` + json5 support |
| directories | 4.x | 5.x | Keep, update |
| toml | 0.5.x | 0.8.x | Keep, update |
| colored | 2.x | 2.x (latest) | Keep |
| tinytemplate | 1.x | latest | Keep |
| shellexpand | 2.x | 3.x | Keep, update |

### RE-004: Error Handling

- All `unwrap()` and `expect()` in user-facing code paths replaced with `?` + `anyhow` context
- `Config::parse()` returns a typed `Result` with context on failure
- `Settings::load()` returns a `Result`, not panicking on parse errors
- Provider methods use `anyhow::Result` throughout

### RE-005: Dev Container

- `Dockerfile` updated to use `rust:1-slim-bookworm` (official image)
- Dev tools installed: `git`, `curl`, `build-essential`, `cargo-llvm-cov`, `cargo-deny`, `cargo-fuzz`
- `devcontainer.json` updated to use `customizations.vscode` (new spec key, replacing `extensions`/`settings`)
- Remove dependency on the legacy `vscode-dev-containers` image

### RE-006: Rust Dev Template Integration

- Clone `git@github.com:ErwanLegrand/rust-dev-template.git` and audit its contents
- Identify and adopt: CI workflow patterns, Cargo workspace layout, Makefile/justfile targets, `.cargo/config.toml`, clippy config, rustfmt config
- Document which features were adopted and which were intentionally skipped

### RE-007: CI Quality Gates

- All existing GitHub Actions workflows updated to use the new binary name and Rust 1.85
- New / updated jobs:
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `cargo deny check` (with `deny.toml` config)
  - `cargo llvm-cov --summary-only` (fail if <80%)
- Workflows run on `push` and `pull_request` to `main`

## Out of Scope

- Dev Containers spec compliance fixes (separate track)
- New CLI commands beyond rename
- Architecture refactoring (separate track)
- Fuzzing harnesses (separate track)

## Acceptance Criteria

- [ ] `cargo build` produces a binary named `devcont`
- [ ] `cargo test` passes with >80% coverage
- [ ] `cargo clippy -- -D warnings` produces zero warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo deny check` passes
- [ ] Dev container builds and launches successfully with the new Dockerfile
- [ ] All GitHub Actions CI jobs green on `main`
- [ ] No `unwrap()`/`expect()` remaining on user-facing code paths
