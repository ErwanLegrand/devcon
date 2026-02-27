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
- [x] Task: Conductor - User Manual Verification 'Phase 1: Re-branding' (Protocol in workflow.md)

## Phase 2: Rust Edition 2024 + Dependency Upgrades

- [x] Task: Write tests for config parsing and settings loading before refactoring
    - [x] Add `tests/config_test.rs` with tests for `Config::parse()` using fixture files
    - [x] Add tests for `Build`, `ShutdownAction` deserialization edge cases
    - [x] Add test for `Settings::load()` with missing config file
    - [x] Verify all new tests pass against current code (GREEN baseline)
- [x] Task: Upgrade to Rust edition 2024 and set MSRV
    - [x] Set `edition = "2024"` in `Cargo.toml`
    - [x] Set `rust-version = "1.85"` in `Cargo.toml`
    - [x] Fix any edition 2024 migration warnings/errors
    - [x] `rust-toolchain.toml` pinned to stable 1.85 (Phase 4 task)
- [x] Task: Upgrade clap from 3.x to 4.x
    - [x] Update `Cargo.toml` dependency
    - [x] Migrate CLI definitions in `src/main.rs` to clap 4 API (`#[command]`, `#[arg]`)
    - [x] Verify `--help` and subcommand behavior unchanged
- [x] Task: Add `anyhow` and `thiserror` dependencies
    - [x] Add to `Cargo.toml`
    - [x] Create `src/error.rs` with typed `Error` enum
- [x] Task: Upgrade remaining dependencies to latest compatible versions
    - [x] `shellexpand` 2.x → 3.x
    - [x] `directories` 4.x → 5.x
    - [x] `toml` 0.5.x → 0.8.x
    - [x] Resolve any API breakage from upgrades
- [x] Task: Replace `unwrap()`/`expect()` with proper error handling
    - [x] `Config::parse()` — propagate json5 parse error with context
    - [x] `Settings::load()` — try_load() returns Result, load() returns default on error with warning
    - [x] `src/commands/start.rs` — shellexpand errors mapped to io::Error
    - [x] `src/commands/rebuild.rs` — same
    - [x] `src/main.rs` — main() returns anyhow::Result<()>
    - [x] `src/provider/docker.rs` — String::from_utf8 uses unwrap_or_default()
    - [x] `src/provider/podman.rs` — same
    - [x] `src/provider/docker_compose.rs` + `podman_compose.rs` — template errors use map_err/?
    - [x] Verify config parsing tests still pass (8/8 passing)
- [x] Task: Run `cargo clippy -- -D warnings` and fix all warnings
- [x] Task: Run `cargo fmt` and commit formatted code
- [x] Task: Conductor - User Manual Verification 'Phase 2: Rust Edition 2024 + Dependency Upgrades' (Protocol in workflow.md)

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
- [x] Task: Conductor - User Manual Verification 'Phase 3: Dev Container Update' (Protocol in workflow.md)

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
- [x] Task: Conductor - User Manual Verification 'Phase 4: Rust Dev Template Integration' (Protocol in workflow.md)

## Phase 5: CI Quality Gates

- [x] Task: Add `cargo-deny` configuration
    - [x] Create `deny.toml` with license allowlist (MIT, Apache, MPL-2.0, etc.)
    - [x] Run `cargo deny check licenses bans` — passes cleanly
    - [x] Note: advisories check scoped to 0.18.x limitation (CVSS 4.0 parse issue); full check restores on Rust 1.88+/cargo-deny 0.19+
- [x] Task: Update GitHub Actions workflows
    - [x] Update `build.yml` — `dtolnay/rust-toolchain@master` pinned to 1.85, `cargo deny check licenses bans`
    - [x] Update `lint.yml` — `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
    - [x] Update `test.yml` — coverage via `taiki-e/install-action@cargo-llvm-cov` + upload to codecov
    - [x] Update `release.yml` — binary name `devcont` throughout
    - [x] All workflows trigger on `push` and `pull_request` to `main`
- [x] Task: Verify all CI jobs pass on current branch
    - [x] `cargo build` ✓
    - [x] `cargo test` ✓ (8/8 passing)
    - [x] `cargo clippy --all-targets -- -D warnings` ✓
    - [x] `cargo fmt --check` ✓
    - [x] `cargo deny check licenses bans` ✓
- [x] Task: Conductor - User Manual Verification 'Phase 5: CI Quality Gates' (Protocol in workflow.md)
