# Plan: Docs — Fill Rustdoc Gaps

## Phase 1: Medium Priority Items

- [ ] Task: Add crate-level `//!` doc to `src/lib.rs`
    - [ ] 2-3 sentence description: what `devcont` is, what the library surface is
    - [ ] `cargo doc --no-deps` must pass
- [ ] Task: Document `print_command` in `src/provider/mod.rs`
    - [ ] `///` doc comment, `# Errors` section (note: cannot error, document that)
    - [ ] `cargo doc --no-deps` must pass
- [ ] Task: Document `start::run` and `rebuild::run`
    - [ ] `///` doc comment describing the command, `# Errors` with failure conditions
    - [ ] `cargo doc --no-deps` must pass
    - [ ] Commit: `docs: add crate-level and command-level rustdoc`

## Phase 2: Low Priority Items — Structs and Fields

- [ ] Task: Document provider structs and `BuildSource` variants
    - [ ] One-line `///` for `Docker`, `Podman`, `DockerCompose`, `PodmanCompose`
    - [ ] One-line `///` for `BuildSource::Dockerfile` and `BuildSource::Image`
    - [ ] `cargo doc --no-deps` must pass
- [ ] Task: Document `Config` and `Build` fields
    - [ ] Per-field `///` doc comments for all public fields
    - [ ] Match descriptions to the Dev Containers spec
    - [ ] `cargo doc --no-deps` must pass
- [ ] Task: Document `exec_host_hook`
    - [ ] Add `///` doc comment explaining host-side execution, `# Errors` section
    - [ ] `cargo doc --no-deps` must pass
    - [ ] Commit: `docs: fill rustdoc gaps on provider structs, Config fields, exec_host_hook`

## Phase 3: Quality Gate

- [ ] Task: Final verification
    - [ ] `cargo doc --no-deps 2>&1 | grep warning` — zero warnings
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
