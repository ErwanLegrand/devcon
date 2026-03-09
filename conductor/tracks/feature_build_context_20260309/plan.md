# Plan: Feature — Full build.context and build.args Support

## Phase 1: Docker and Podman build() Updates

- [ ] Task: Update `Docker::build()` to use context and build args
    - [ ] Pass `config.build.context` as build context argument when set
    - [ ] Append `--build-arg KEY=VALUE` for each `build.args` entry
    - [ ] `cargo build` must pass
- [ ] Task: Update `Podman::build()` identically
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(provider): pass build.context and build.args to Docker/Podman build`

## Phase 2: Compose Provider Updates

- [ ] Task: Update compose override template for build args
    - [ ] Add `build.args` map to compose override YAML when `build.args` is set
    - [ ] Update `TemplateContext` to include `build_args: HashMap<String, String>`
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(provider): inject build.args into compose override template`

## Phase 3: Tests

- [ ] Task: Unit tests for build arg injection
    - [ ] Docker build args appear as `--build-arg KEY=VALUE` in command args
    - [ ] Empty build args → no `--build-arg` flags
    - [ ] Compose override includes build args map
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): build.context and build.args coverage`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
