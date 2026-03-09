# Plan: Fix — Podman Providers Missing mounts and remoteEnv Support

## Phase 1: Implement mounts in Podman::create()

- [ ] Task: Add mounts to `Podman::create()`
    - [ ] For each entry in `config.mounts`, append `--mount <entry>` or `-v <entry>` to args
    - [ ] `cargo build` must pass
- [ ] Task: Add mounts and remoteEnv to `PodmanCompose` override template
    - [ ] Inject `mounts` as volume entries in compose override YAML
    - [ ] Inject `remoteEnv` as environment variables in compose override YAML
    - [ ] Update `TemplateContext` to include these fields
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(provider): add mounts and remoteEnv support to Podman providers`

## Phase 2: Tests

- [ ] Task: Unit tests for Podman mounts parity
    - [ ] Podman create args include all mounts from config
    - [ ] PodmanCompose override includes mounts and remoteEnv
    - [ ] Empty mounts list → no extra args
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): Podman mounts and remoteEnv parity`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
