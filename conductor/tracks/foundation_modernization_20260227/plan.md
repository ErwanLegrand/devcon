# Plan: Foundation Modernization

## Phase 1: Re-branding (devcon → devcont)

- [x] Task: Write regression tests capturing current binary name, container prefix, and config path behavior
    - [x] Add `tests/branding_test.rs` asserting `safe_name()` output format
    - [x] Add unit test for `Settings::load()` default provider
    - [x] Verify tests pass against current code (GREEN baseline)
- [x] Task: Rename package and binary in `Cargo.toml`
    - [x] Set `name = "devcont"` in `[package]`
    - [x] Add `[[bin]]` section with `name = "devcont"`, `path = "src/main.rs"`
- [x] Task: Update container name prefix
    - [x] Change `devcon-` prefix to `devcont-` in `Config::safe_name()`
    - [x] Update branding tests to expect new prefix
- [x] Task: Update all internal string references
    - [x] Grep for `devcon` references in source; update each
    - [x] Update `Settings::load()` project dirs identifier (`"devcon"` → `"devcont"`)
- [x] Task: Update README.md
    - [x] Replace all `devcon` command references with `devcont`
    - [x] Update installation section
- [x] Task: Run full test suite and verify branding tests pass
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
    - [ ] Create `rust-toolchain.toml` pinned to stable 1.85
- [ ] Task: Upgrade clap from 3.x to 4.x
    - [ ] Update `Cargo.toml` dependency
    - [ ] Migrate CLI definitions in `src/main.rs` to clap 4 API
    - [ ] Verify `--help` and subcommand behavior unchanged
- [ ] Task: Add `anyhow` and `thiserror` dependencies
    - [ ] Add to `Cargo.toml`
    - [ ] Create `src/error.rs` with typed `Error` enum (from template pattern)
    - [ ] Create `src/prelude.rs` re-exporting common types
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

- [x] Task: Write a smoke test / build validation script for the dev container
    - [x] Add `scripts/validate-devcontainer.sh` that checks Dockerfile lints
- [x] Task: Update `Dockerfile` to official rust image
    - [x] Change base image to `rust:1-slim-bookworm`
    - [x] Install required system packages (`git`, `curl`, `build-essential`, `pkg-config`, `libssl-dev`)
    - [x] Install dev tools: `cargo-llvm-cov`, `cargo-deny`
    - [x] Remove legacy vscode-dev-containers artifacts
- [x] Task: Update `devcontainer.json`
    - [x] Replace deprecated `extensions` key with `customizations.vscode.extensions`
    - [x] Replace deprecated `settings` key with `customizations.vscode.settings`
    - [x] Update `remoteUser` to `devcont`
    - [x] Verify `devcontainer.json` is valid JSON5
- [x] Task: Enrich `devcontainer.json` with template patterns
    - [x] Add `containerEnv`: `RUST_BACKTRACE=1`, `RUST_LOG=info`
    - [x] Add `postCreateCommand` pointing to `.devcontainer/post-create.sh`
    - [x] Expand VS Code extensions and settings from template audit
- [x] Task: Create `.devcontainer/post-create.sh` (adapted from template)
    - [x] Install cargo tools, configure git, run `cargo check`, display welcome
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Dev Container Update' (Protocol in workflow.md)

## Phase 4: Rust Dev Template Integration

- [x] Task: Audit the rust-dev-template repository
    - [x] Clone `git@github.com:ErwanLegrand/rust-dev-template.git` to a temp location
    - [x] Read and document the structure and features of the template
    - [x] Produce a written list of features to adopt vs skip (saved to `conductor/tracks/foundation_modernization_20260227/template-audit.md`)
- [x] Task: Adopt `rust-toolchain.toml` from template (adapted for stable 1.85)
    - [x] Create `rust-toolchain.toml` with `channel = "1.85"`, components listed
- [x] Task: Adopt `src/error.rs` and `src/prelude.rs` pattern
    - [x] Create `src/error.rs` with typed Error enum using `thiserror`
    - [x] (Merged into Phase 2 error handling task)
- [x] Task: Adopt `.pre-commit-config.yaml` from template
    - [x] Copy and adapt the pre-commit config (removed cargo-outdated, requirements-txt-fixer)
- [ ] Task: Adopt `.cargo/config.toml` from template
    - [ ] Copy the xtask alias (deferred — xtask not added yet)
- [x] Task: Create `DEPENDENCIES.md`
    - [x] Document all runtime and dev dependencies and rationale
- [x] Task: Enrich devcontainer with template patterns
    - [x] `containerEnv`, `postCreateCommand`, expanded VS Code settings
    - [x] `.devcontainer/post-create.sh` created and made executable
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
