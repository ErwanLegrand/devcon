# Rust Dev Template Audit

## Template Source
- Repo: git@github.com:ErwanLegrand/rust-dev-template.git
- Audited: 2026-02-27

## Summary

The template is a comprehensive Rust development environment scaffold. It features a Cargo workspace
with a main library crate plus an `xtask` crate for CI/CD orchestration, pre-commit hooks, a rich
devcontainer configuration, and a `post-create.sh` script. It targets Rust 2024 edition with `thiserror`
for error handling and `tracing` for structured logging.

---

## Feature Analysis

### Workspace Structure (`Cargo.toml`)
- **Status:** SKIP
- **Rationale:** The template uses a Cargo workspace (main crate + `xtask`). `devcont` is a binary CLI, not a library. Adding a workspace would complicate the build without benefit unless we add `cargo-devcont` as a separate crate.
- **Action:** Keep single-crate layout for now. Revisit if `cargo devcont` requires a separate crate.

### Rust Edition & MSRV
- **Status:** ADOPT
- **Rationale:** Template uses `edition = "2024"`. This aligns exactly with our plan. Note: the template's `rust-toolchain.toml` specifies `channel = "nightly"` because the template was written before 1.85 stabilized Rust 2024. We use stable 1.85+.
- **Action:** Set `edition = "2024"` and `rust-version = "1.85"` in `Cargo.toml`. Create `rust-toolchain.toml` pinned to stable channel.

### Error Handling Pattern (`src/error.rs` + `src/prelude.rs`)
- **Status:** ADOPT
- **Rationale:** Clean separation of error types (`thiserror`) with a prelude re-exporting common types. Exactly what we need.
- **Action:** Create `src/error.rs` with a typed `Error` enum using `thiserror`. Create `src/prelude.rs` re-exporting `Error`, `Result`, and common std items. Use `anyhow::Result` in `main()` for top-level error handling.

### `xtask` Crate
- **Status:** ADAPT
- **Rationale:** The xtask has comprehensive commands: `build`, `test`, `quality`, `lint`, `fmt`, `doc-check`, `audit`, `ci`, `setup`, `check`. This is excellent for developer ergonomics via `cargo xtask <cmd>`. However, the current `ci.rs` is a stub and `quality.rs` uses `cargo audit` (which we're replacing with `cargo deny`).
- **Action:** Add a minimal `xtask` crate with: `check` (fmt + clippy + test), `ci` (fmt-check + clippy -D warnings + test + deny + coverage), `lint`, `fmt`. Skip tokio dependency for xtask — use `std::process::Command` or `duct` directly. Add workspace in a follow-up track, not this one — too much scope change.

### `.cargo/config.toml`
- **Status:** ADOPT
- **Rationale:** Defines the `cargo xtask` alias. Small, useful.
- **Action:** Copy as-is. Extend later if needed (e.g., `linker` flags, `target` defaults).

### `rust-toolchain.toml`
- **Status:** ADAPT
- **Rationale:** Template uses `channel = "nightly"` (written before Rust 2024 stabilized). We use stable 1.85+.
- **Action:** Create `rust-toolchain.toml` at project root:
  ```toml
  [toolchain]
  channel = "1.85"
  components = ["rustfmt", "clippy", "rust-analyzer", "llvm-tools-preview"]
  ```

### `.pre-commit-config.yaml`
- **Status:** ADOPT
- **Rationale:** Excellent config: rustfmt, clippy, cargo-check, cargo-test, cargo-audit, cargo-deny, conventional-commit-msg, general file checks (trailing whitespace, CRLF, large files, private key detection, YAML/TOML/JSON validation). Exactly what we want.
- **Action:** Copy the config. Adjust clippy flags (keep `-D warnings -W clippy::pedantic`). Remove `cargo-outdated` hook (too noisy in CI — run manually). Remove `requirements-txt-fixer` (no Python).

### Dev Container (`devcontainer.json`)
- **Status:** ALREADY DONE (Worker B implemented Phase 3)
- **Rationale:** Template has good `containerEnv` (RUST_BACKTRACE=1, RUST_LOG=info), good VS Code settings, and `post-create.sh`. Our Worker B already updated devcontainer.json.
- **Action:** Enrich the existing devcontainer.json with:
  - `containerEnv`: `RUST_BACKTRACE=1`, `RUST_LOG=info`
  - `postCreateCommand`: add `.devcontainer/post-create.sh`
  - Add `rust-analyzer.checkOnSave.extraArgs` to enable pedantic clippy in the IDE
  - Expand VS Code extensions list (GitHub Actions, spell checker)

### `post-create.sh`
- **Status:** ADAPT
- **Rationale:** Good script for setting up the dev environment after container creation. Remove nightly references, simplify.
- **Action:** Create `.devcontainer/post-create.sh` adapted for devcont: install cargo tools, configure git, run `cargo check`, display welcome message.

### `DEPENDENCIES.md`
- **Status:** ADOPT
- **Rationale:** Documents why each dependency exists. Good transparency practice.
- **Action:** Create `DEPENDENCIES.md` documenting our dependency choices.

### `pre-commit` Conventional Commits Enforcement
- **Status:** ADOPT
- **Rationale:** Enforces conventional commit format (feat, fix, etc.) at commit time. Already in our workflow.
- **Action:** Included in the `.pre-commit-config.yaml` adoption above.

### Dependency: `tracing` + `tracing-subscriber`
- **Status:** SKIP (for now)
- **Rationale:** `devcont` is a CLI that prints commands to stdout using `colored`. Structured logging with `tracing` would be beneficial for verbose mode, but it's out of scope for this foundation track.
- **Action:** Defer to a future architecture improvement track.

### Dependency: `proptest` / `criterion`
- **Status:** SKIP
- **Rationale:** Property-based testing and benchmarking are valuable but not part of this foundation track.
- **Action:** Defer to the testing/fuzzing track.

---

## Key Adoptions (Priority Order)

1. `rust-toolchain.toml` — pin to stable 1.85, add components
2. `src/error.rs` + `src/prelude.rs` — typed error handling pattern
3. `.pre-commit-config.yaml` — pre-commit hooks (adapted)
4. `.cargo/config.toml` — xtask alias
5. `.devcontainer/post-create.sh` — dev container setup script (adapted)
6. `DEPENDENCIES.md` — dependency documentation
7. Enrich `devcontainer.json` with `containerEnv` and `postCreateCommand`

## Skipped Items

- Cargo workspace (single-crate binary doesn't need it yet)
- `xtask` crate (scope for a separate track or when `cargo devcont` is added)
- `tracing` / `tracing-subscriber` (deferred to architecture track)
- `proptest` / `criterion` (deferred to testing track)
- `cargo-outdated` pre-commit hook (too noisy, run manually)
- Nightly toolchain channel (use stable 1.85)
- Docker-in-docker, Node.js devcontainer features (not needed)
