# Plan: Fix — DockerCompose exists() Uses Wrong Container Name Source

## Phase 1: Fix exists() Using Compose ps

- [ ] Task: Fix `DockerCompose::exists()`
    - [ ] Replace guessed container name with `docker compose -f <file> ps --format json <service>`
    - [ ] Parse output to check if service has a container
    - [ ] `cargo build` must pass
- [ ] Task: Fix `PodmanCompose::exists()`
    - [ ] Apply equivalent fix using `podman-compose ps`
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(provider): use compose ps to detect container existence in exists()`

## Phase 2: Tests

- [ ] Task: Unit tests for compose ps output parsing
    - [ ] Populated ps output → exists() returns true
    - [ ] Empty ps output → exists() returns false
    - [ ] Malformed output → exists() returns false (no panic)
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(provider): DockerCompose/PodmanCompose exists() output parsing`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
