# Plan: Feature — Full build.context and build.args Support

## Phase 1: Docker and Podman build() Updates

- [x] Task: Update `Docker::build()` to use context and build args
    - [x] Pass `config.build.context` as build context argument when set
    - [x] Append `--build-arg KEY=VALUE` for each `build.args` entry
    - [x] `cargo build` must pass
- [x] Task: Update `Podman::build()` identically
    - [x] `cargo build` must pass
    - [x] Commit: `feat(provider): wire build.context and build.args across all providers`

## Phase 2: Compose Provider Updates

- [x] Task: Update compose override template for build args
    - [x] Add `build.args` map to compose override YAML when `build.args` is set
    - [x] Update `TemplateContext` to include `build_args: Vec<TemplateEntry>`
    - [x] `cargo build` must pass
    - [x] Commit: (included in Phase 1 commit)

## Phase 3: Tests

- [x] Task: Unit tests for build arg injection
    - [x] resolve_build_context uses directory when no context set
    - [x] resolve_build_context joins relative context with directory
    - [x] resolve_build_context returns absolute context as-is
    - [x] Compose override includes build args map
    - [x] Compose override without build_args omits build section
    - [x] `cargo test` must pass
    - [x] Commit: (included in Phase 1 commit)

## Phase 4: Quality Gate

- [x] Task: Full quality gate
    - [x] `cargo test` — 103 tests pass
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
