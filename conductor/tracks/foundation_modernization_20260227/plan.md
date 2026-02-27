# Plan: Foundation Modernization

## Phase 1: Re-branding (devcon → devcont)

- [ ] Task: Write regression tests capturing current binary name, container prefix, and config path behavior
    - [ ] Add `tests/branding_test.rs` asserting `safe_name()` output format
    - [ ] Add unit test for `Settings::load()` default provider
    - [ ] Verify tests pass against current code (GREEN baseline)
- [ ] Task: Rename package and binary in `Cargo.toml`
    - [ ] Set `name = "devcont"` in `[package]`
    - [ ] Add `[[bin]]` section with `name = "devcont"`, `path = "src/main.rs"`
- [ ] Task: Update container name prefix
    - [ ] Change `devcon-` prefix to `devcont-` in `Config::safe_name()`
    - [ ] Update branding tests to expect new prefix
- [ ] Task: Update all internal string references
    - [ ] Grep for `devcon` references in source; update each
    - [ ] Update `Settings::load()` project dirs identifier (`"devcon"` → `"devcont"`)
- [ ] Task: Update README.md
    - [ ] Replace all `devcon` command references with `devcont`
    - [ ] Update installation section
- [ ] Task: Run full test suite and verify branding tests pass
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Re-branding' (Protocol in workflow.md)

## Phase 2: Rust Edition 2024 + Dependency Upgrades

- [ ] Task: Write tests for config parsing and settings loading before refactoring
    - [ ] Add `tests/config_test.rs` with tests for `Config::parse()` using fixture files
    - [ ] Add tests for `Build`, `ShutdownAction` deserialization edge cases
    - [ ] Add test for `Settings::load()` with missing config file
    - [ ] Verify all new tests pass against current code (GREEN baseline)
- [ ] Task: Upgrade to Rust edition 2024 and set MSRV
    - [ ] Set `edition = "2024"` in `Cargo.toml`
    - [ ] Set `rust-version = "1.85"` in `Cargo.toml`
    - [ ] Fix any edition 2024 migration warnings/errors (`cargo fix --edition`)
- [ ] Task: Upgrade clap from 3.x to 4.x
    - [ ] Update `Cargo.toml` dependency
    - [ ] Migrate CLI definitions in `src/main.rs` to clap 4 API
    - [ ] Verify `--help` and subcommand behavior unchanged
- [ ] Task: Add `anyhow` and `thiserror` dependencies
    - [ ] Add to `Cargo.toml`
- [ ] Task: Upgrade remaining dependencies to latest compatible versions
    - [ ] `shellexpand` 2.x → 3.x
    - [ ] `directories` 4.x → 5.x
    - [ ] `toml` 0.5.x → 0.8.x
    - [ ] `serde`, `serde_json`, `json5`, `colored`, `tinytemplate` — bump to latest
    - [ ] Resolve any API breakage from upgrades
- [ ] Task: Replace `unwrap()`/`expect()` with proper error handling
    - [ ] `Config::parse()` — propagate json5 parse error with context
    - [ ] `Settings::load()` — return `anyhow::Result<Settings>`, propagate toml errors
    - [ ] `src/commands/start.rs` — propagate errors up
    - [ ] `src/commands/rebuild.rs` — propagate errors up
    - [ ] `src/main.rs` — handle top-level errors with user-friendly message + non-zero exit
    - [ ] `src/provider/docker.rs` — replace `String::from_utf8(...).unwrap()`
    - [ ] `src/provider/podman.rs` — same
    - [ ] Verify config parsing tests still pass
- [ ] Task: Run `cargo clippy -- -D warnings` and fix all warnings
- [ ] Task: Run `cargo fmt` and commit formatted code
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Rust Edition 2024 + Dependency Upgrades' (Protocol in workflow.md)

## Phase 3: Dev Container Update

- [ ] Task: Write a smoke test / build validation script for the dev container
    - [ ] Add `scripts/validate-devcontainer.sh` that checks Dockerfile lints
- [ ] Task: Update `Dockerfile` to official rust image
    - [ ] Change base image to `rust:1-slim-bookworm`
    - [ ] Install required system packages (`git`, `curl`, `build-essential`, `pkg-config`, `libssl-dev`)
    - [ ] Install dev tools: `cargo-llvm-cov`, `cargo-deny`
    - [ ] Remove legacy vscode-dev-containers artifacts
- [ ] Task: Update `devcontainer.json`
    - [ ] Replace deprecated `extensions` key with `customizations.vscode.extensions`
    - [ ] Replace deprecated `settings` key with `customizations.vscode.settings`
    - [ ] Update `remoteUser` if needed
    - [ ] Verify `devcontainer.json` is valid JSON5
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Dev Container Update' (Protocol in workflow.md)

## Phase 4: Rust Dev Template Integration

- [ ] Task: Audit the rust-dev-template repository
    - [ ] Clone `git@github.com:ErwanLegrand/rust-dev-template.git` to a temp location
    - [ ] Read and document the structure and features of the template
    - [ ] Produce a written list of features to adopt vs skip (save to `conductor/tracks/foundation_modernization_20260227/template-audit.md`)
- [ ] Task: Adopt CI workflow patterns from template
    - [ ] Compare GitHub Actions workflows and adopt improvements
- [ ] Task: Adopt Cargo and toolchain configuration from template
    - [ ] Copy/merge `.cargo/config.toml` if present
    - [ ] Copy/merge `rustfmt.toml` if present (ensure `max_width = 100`)
    - [ ] Copy/merge `clippy.toml` or `#![warn(...)]` attrs if present
- [ ] Task: Adopt Makefile or justfile targets from template (if present)
- [ ] Task: Adopt any other relevant structure (workspace layout, deny.toml, etc.)
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Rust Dev Template Integration' (Protocol in workflow.md)

## Phase 5: CI Quality Gates

- [ ] Task: Add `cargo-deny` configuration
    - [ ] Create `deny.toml` with license allowlist and advisory database check
    - [ ] Run `cargo deny check` locally and fix any issues
- [ ] Task: Update GitHub Actions workflows
    - [ ] Update `build.yml` — use `rust-version` from `Cargo.toml`, add `cargo deny check`
    - [ ] Update `lint.yml` — add `cargo fmt --check`, ensure clippy uses `-D warnings`
    - [ ] Update `test.yml` — add `cargo llvm-cov --summary-only`, fail if <80%
    - [ ] Update `release.yml` — use new binary name `devcont`
    - [ ] Ensure all workflows trigger on `push` and `pull_request` to `main`
- [ ] Task: Verify all CI jobs pass on current branch
    - [ ] `cargo build` ✓
    - [ ] `cargo test` ✓
    - [ ] `cargo clippy -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check` ✓
    - [ ] Coverage ≥80% ✓
- [ ] Task: Conductor - User Manual Verification 'Phase 5: CI Quality Gates' (Protocol in workflow.md)
