# Plan: podman-compose Provider Completion

## Phase 1: Bug Fixes

- [x] Task: Fix `PodmanCompose::running()` (FR-001)
    - [x] Remove `--format {{.ID}}` argument from `podman ps` call in `running()`
    - [x] Add `--filter` `status=running` to the `podman ps` call
    - [x] `cargo test --test integration` must pass
    - [x] Commit: `fix(provider): fix PodmanCompose::running() flag conflict`
- [ ] Task: Fix `attach()` shell in both compose providers (FR-002)
    - [ ] Add `shell: String` field to `DockerCompose` struct in `docker_compose.rs`
    - [ ] Add `shell: String` field to `PodmanCompose` struct in `podman_compose.rs`
    - [ ] Replace hardcoded `"zsh"` with `&self.shell` in both `attach()` impls
    - [ ] Update `build_provider` in `src/devcontainers/mod.rs` to pass `shell:
          "sh".to_string()` for both compose providers
    - [ ] Update `load_compose_provider` and `load_podman_compose_provider` in
          `tests/integration.rs` to include `shell: "sh".to_string()`
    - [ ] `cargo test --test integration` must pass
    - [ ] Commit: `fix(provider): replace hardcoded zsh with configurable shell in compose attach`
- [ ] Task: Fix `PodmanCompose::cp()` container ID resolution (FR-003)
    - [ ] Change `podman ps` output processing to take only the first non-empty
          line (`.lines().find(|l| !l.trim().is_empty())`)
    - [ ] Return `Ok(false)` when no matching container ID is found
    - [ ] `cargo test --test integration` must pass
    - [ ] Commit: `fix(provider): fix PodmanCompose::cp() for scaled services`
- [ ] Task: Remove `--rmi all` from `PodmanCompose::rm()` (FR-005)
    - [ ] Delete the `--rmi` and `all` args from the `podman-compose down` invocation
    - [ ] `cargo test --test integration` must pass
    - [ ] Commit: `fix(provider): remove --rmi all from PodmanCompose::rm()`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Bug Fixes' (Protocol in workflow.md)

## Phase 2: remote_env Injection via Compose Override (FR-004)

- [ ] Task: Extend compose override template and `create_compose_override`
    - [ ] Read `templates/docker-compose.yml` to understand the current template
          structure
    - [ ] Add `environment:` block to the template, rendering `env_vars` entries
          as `KEY: VALUE` pairs under the service
    - [ ] Update `TemplateContext` in `src/provider/utils.rs` to add
          `env_vars: Vec<TemplateEntry>`
    - [ ] Update `create_compose_override` signature to accept
          `env_vars: &[(String, String)]` and populate `TemplateContext`
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(provider): extend compose override template with environment vars`
- [ ] Task: Wire `remote_env` into compose provider structs and construction
    - [ ] Add `env_vars: Vec<(String, String)>` field to `DockerCompose` struct
    - [ ] Add `env_vars: Vec<(String, String)>` field to `PodmanCompose` struct
    - [ ] Update `DockerCompose::create_docker_compose()` to pass `&self.env_vars`
          to `create_compose_override`
    - [ ] Update `PodmanCompose::create_docker_compose()` to pass `&self.env_vars`
          to `create_compose_override`
    - [ ] Update `build_provider` in `src/devcontainers/mod.rs` to populate
          `env_vars` from `config.remote_env` (convert `HashMap<String,String>` to
          sorted `Vec<(String,String)>`)
    - [ ] Update `load_compose_provider` and `load_podman_compose_provider` in
          `tests/integration.rs` to include `env_vars: vec![]`
    - [ ] `cargo test --test integration` must pass
    - [ ] Commit: `feat(provider): wire remote_env into compose override via env_vars field`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: remote_env Injection' (Protocol in workflow.md)

## Phase 3: Tests and Quality Gate (FR-006)

- [ ] Task: Add integration test for compose exec with `sh` shell
    - [ ] Add `test_docker_compose_exec_with_sh` test that verifies `exec("echo ok")`
          succeeds on the compose fixture (exercises the same provider path as attach)
    - [ ] Ensure the test uses `shell: "sh"` in the fixture provider
    - [ ] `cargo test --test integration` must pass
    - [ ] Commit: `test(integration): add compose exec test verifying sh shell path`
- [ ] Task: Run full quality gate
    - [ ] `cargo test` ✓
    - [ ] `cargo test --test integration` ✓
    - [ ] `cargo clippy --all-targets -- -D warnings` ✓
    - [ ] `cargo fmt --check` ✓
    - [ ] `cargo deny check licenses bans advisories` ✓
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Tests and Quality Gate' (Protocol in workflow.md)
