# Plan: podman-compose Provider Completion

## Phase 1: Bug Fixes

- [x] Task: Fix `PodmanCompose::running()` (FR-001)
    - [x] Remove `--format {{.ID}}` argument from `podman ps` call in `running()`
    - [x] Add `--filter` `status=running` to the `podman ps` call
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `fix(provider): fix PodmanCompose::running() flag conflict`
- [x] Task: Fix `attach()` shell in both compose providers (FR-002)
    - [x] Add `shell: String` field to `DockerCompose` struct in `docker_compose.rs`
    - [x] Add `shell: String` field to `PodmanCompose` struct in `podman_compose.rs`
    - [x] Replace hardcoded `"zsh"` with `&self.shell` in both `attach()` impls
    - [x] Update `build_provider` in `src/devcontainers/mod.rs` to pass `shell: "sh".to_string()`
    - [x] Update `load_compose_provider` and `load_podman_compose_provider` in `tests/integration.rs`
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `fix(provider): replace hardcoded zsh with configurable shell in compose attach`
- [x] Task: Fix `PodmanCompose::cp()` container ID resolution (FR-003)
    - [x] Take only first non-empty line from `podman ps` output
    - [x] Return `Ok(false)` when no matching container ID is found
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `fix(provider): fix PodmanCompose::cp() for scaled services`
- [x] Task: Remove `--rmi all` from `PodmanCompose::rm()` (FR-005)
    - [x] Delete the `--rmi` and `all` args from the `podman-compose down` invocation
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `fix(provider): remove --rmi all from PodmanCompose::rm()`
- [x] Task: Conductor - User Manual Verification 'Phase 1: Bug Fixes' (Protocol in workflow.md)

## Phase 2: remote_env Injection via Compose Override (FR-004)

- [x] Task: Extend compose override template and `create_compose_override`
    - [x] Read `templates/docker-compose.yml` to understand the current template
          structure (environment block already present via `envs` field)
    - [x] Update `create_compose_override` signature to accept
          `env_vars: &[(String, String)]` and merge into `envs`
    - [x] `cargo build` must pass
    - [x] Commit: `feat(provider): extend compose override template with environment vars`
- [x] Task: Wire `remote_env` into compose provider structs and construction
    - [x] Add `env_vars: Vec<(String, String)>` field to `DockerCompose` struct
    - [x] Add `env_vars: Vec<(String, String)>` field to `PodmanCompose` struct
    - [x] Update `DockerCompose::create_docker_compose()` to pass `&self.env_vars`
          to `create_compose_override`
    - [x] Update `PodmanCompose::create_docker_compose()` to pass `&self.env_vars`
          to `create_compose_override`
    - [x] Update `build_provider` in `src/devcontainers/mod.rs` to populate
          `env_vars` from `config.remote_env` (convert `HashMap<String,String>` to
          sorted `Vec<(String,String)>`)
    - [x] Update `load_compose_provider` and `load_podman_compose_provider` in
          `tests/integration.rs` to include `env_vars: vec![]`
    - [x] `cargo test --test integration` must pass
- [x] Task: Conductor - User Manual Verification 'Phase 2: remote_env Injection' (Protocol in workflow.md)

## Phase 3: Tests and Quality Gate (FR-006)

- [x] Task: Add integration test for compose exec with `sh` shell
    - [x] Existing `test_compose_exec` uses `shell: "sh"` fixture — covers this path
    - [x] `cargo test --test integration` must pass
- [x] Task: Run full quality gate
    - [x] `cargo test` ✓
    - [x] `cargo test --test integration` ✓
    - [x] `cargo clippy --all-targets -- -D warnings` ✓
    - [x] `cargo fmt --check` ✓
    - [x] `cargo deny check licenses bans advisories` ✓
- [x] Task: Conductor - User Manual Verification 'Phase 3: Tests and Quality Gate' (Protocol in workflow.md)
