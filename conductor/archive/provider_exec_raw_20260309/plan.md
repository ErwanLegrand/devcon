# Plan: Provider exec_raw — Injection-Safe Direct Exec for Many Hooks

## Phase 1: Add exec_raw to Provider trait and all implementations

- [x] Task: Add `exec_raw` to `Provider` trait
    - [x] In `src/provider/mod.rs`, add `fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool>`
    - [x] `cargo build` must pass (trait implementations will fail until updated)
- [x] Task: Implement `exec_raw` for `Docker`
    - [x] In `src/provider/docker.rs`, add `exec_raw` that runs `docker exec -u <user> -w <workspace> <name> <prog> <args...>` (no `sh -c`)
    - [x] `cargo build` must pass
- [x] Task: Implement `exec_raw` for `Podman`
    - [x] In `src/provider/podman.rs`, add `exec_raw` mirroring Docker
    - [x] `cargo build` must pass
- [x] Task: Implement `exec_raw` for `DockerCompose`
    - [x] In `src/provider/docker_compose.rs`, add `exec_raw` that runs `docker compose exec -u <user> -w <workspace> <service> <prog> <args...>`
    - [x] `cargo build` must pass
- [x] Task: Implement `exec_raw` for `PodmanCompose`
    - [x] In `src/provider/podman_compose.rs`, add `exec_raw` mirroring DockerCompose
    - [x] `cargo build` must pass
- [x] Task: Update `exec_hook` to use `exec_raw` for Many variant
    - [x] In `src/devcontainers/mod.rs`, change `Many(parts)` arm to use `to_exec_parts()` and call `provider.exec_raw(prog, args)`
    - [x] `cargo build` must pass
    - [x] Commit: `feat(provider): add exec_raw for injection-safe direct exec of Many lifecycle hooks`

## Phase 2: Tests

- [x] Task: Write unit tests for `exec_hook` with Many variant
    - [x] Add unit tests verifying that `exec_hook` with `Many(["prog", "arg with space"])` calls `exec_raw` not `exec`
    - [x] `cargo test` must pass
    - [x] Commit: `test(devcontainer): verify exec_hook Many uses exec_raw`

## Phase 3: Quality Gate

- [x] Task: Run full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
