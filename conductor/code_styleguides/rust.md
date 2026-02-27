# Rust Code Style Guide

## Edition & MSRV

- Use **Rust edition 2024**
- MSRV: **1.85+**
- Declare in `Cargo.toml`: `edition = "2024"`, `rust-version = "1.85"`

## Naming Conventions

- Types, traits, enums: `UpperCamelCase`
- Functions, methods, variables, modules: `snake_case`
- Constants and statics: `SCREAMING_SNAKE_CASE`
- Lifetimes: short lowercase (`'a`, `'buf`)
- Avoid abbreviations unless universally understood (`cfg`, `dir`, `cmd`)

## Error Handling

- **Never** use `unwrap()` or `expect()` on user-facing code paths
- Use `anyhow::Result` for binary/application code
- Use `thiserror` for typed errors in library/domain code
- Propagate with `?` operator; add context with `.context("what failed")`
- Validate inputs at system boundaries (CLI args, file reads, env vars)

## Immutability

- Prefer `let` over `let mut`; only use `mut` when mutation is necessary
- Prefer returning new values over mutating arguments in place
- Use `Clone` judiciously; avoid cloning in hot paths

## Module Structure

- One concept per file; files stay under ~400 lines
- Organize by domain, not by type (e.g., `provider/docker.rs` not `structs/docker.rs`)
- Keep `pub` surface minimal — only expose what callers need
- Use `pub(crate)` for crate-internal visibility

## Functions

- Keep functions under 50 lines
- Single responsibility — one function, one job
- Avoid deep nesting (>3 levels); extract into helper functions or use early returns

## Types

- Prefer newtypes over raw primitives for domain values (e.g., `ContainerName(String)`)
- Derive `Debug` on all types; derive `Clone` only when needed
- Use `Default` trait instead of custom constructors where appropriate
- Use `Option` for optional values, never sentinel values (empty string, -1, etc.)

## Traits

- Define traits for testability and abstraction (e.g., `Provider` trait for Docker/Podman)
- Prefer small, focused traits over large monolithic ones
- Implement standard traits (`Display`, `From`, `TryFrom`) where appropriate

## Testing

- All modules must have a `#[cfg(test)]` section
- Use `#[test]` for unit tests, integration tests in `tests/`
- Mock external dependencies (container engines) via trait objects
- Target >80% line coverage measured with `cargo-llvm-cov`
- Fuzz entry points: `devcontainer.json` parser, config parser

## Documentation

- All `pub` items must have `///` doc comments
- Doc comments describe *what* and *why*, not *how*
- Include examples in doc comments for non-trivial public APIs (`# Examples` section)
- Use `#[doc = include_str!("../README.md")]` on the crate root if appropriate

## Clippy

- Run `cargo clippy -- -D warnings` in CI; zero warnings policy
- Enable `#![warn(clippy::pedantic)]` in `main.rs`/`lib.rs`
- Suppress individual lints with `#[allow(clippy::...)]` + a comment explaining why

## Formatting

- Use `rustfmt` with default settings (enforced in CI)
- Maximum line length: 100 characters (`max_width = 100` in `rustfmt.toml`)

## Dependencies

- Audit with `cargo-deny` in CI (licenses, security advisories)
- Keep dependency count minimal; prefer std over external crates for simple tasks
- Pin MSRV-compatible versions in `Cargo.toml`

## Security

- Never interpolate user input directly into shell commands — use `Command::arg()` always
- Validate and canonicalize all file paths before use
- Do not log sensitive environment variables
- Use `OsStr`/`OsString` for paths that may contain non-UTF-8 bytes
