# Plan: Provider exec_raw — Injection-Safe Direct Exec for Many Hooks

## Phase 1: Add exec_raw to Provider trait and all implementations

- [ ] Task: Add `exec_raw` to `Provider` trait
    - [ ] In `src/provider/mod.rs`, add `fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool>`
    - [ ] `cargo build` must pass (trait implementations will fail until updated)
- [ ] Task: Implement `exec_raw` for `Docker`
    - [ ] In `src/provider/docker.rs`, add `exec_raw` that runs `docker exec -u <user> -w <workspace> <name> <prog> <args...>` (no `sh -c`)
    - [ ] `cargo build` must pass
- [ ] Task: Implement `exec_raw` for `Podman`
    - [ ] In `src/provider/podman.rs`, add `exec_raw` mirroring Docker
    - [ ] `cargo build` must pass
- [ ] Task: Implement `exec_raw` for `DockerCompose`
    - [ ] In `src/provider/docker_compose.rs`, add `exec_raw` that runs `docker compose exec -u <user> -w <workspace> <service> <prog> <args...>`
    - [ ] `cargo build` must pass
- [ ] Task: Implement `exec_raw` for `PodmanCompose`
    - [ ] In `src/provider/podman_compose.rs`, add `exec_raw` mirroring DockerCompose
    - [ ] `cargo build` must pass
- [ ] Task: Update `exec_hook` to use `exec_raw` for Many variant
    - [ ] In `src/devcontainers/mod.rs`, change `Many(parts)` arm to use `to_exec_parts()` and call `provider.exec_raw(prog, args)`
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(provider): add exec_raw for injection-safe direct exec of Many lifecycle hooks`

## Phase 2: Tests

- [ ] Task: Write unit tests for `exec_hook` with Many variant
    - [ ] Add unit tests verifying that `exec_hook` with `Many(["prog", "arg with space"])` calls `exec_raw` not `exec`
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): verify exec_hook Many uses exec_raw`

## Phase 3: Quality Gate

- [ ] Task: Run full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
