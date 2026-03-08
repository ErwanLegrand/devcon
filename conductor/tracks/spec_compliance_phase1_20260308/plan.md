# Plan: Dev Containers Spec Compliance — Phase 1

## Phase 1: `image` Field Support (FR-001)

- [x] Task: Add `image` field to `Config` and update `build_provider`
    - [ ] Add `image: Option<String>` field to `Config` struct in
          `src/devcontainers/config.rs` with `#[serde(rename_all = "camelCase")]`
    - [ ] In `build_provider` (`src/devcontainers/mod.rs`), add a third branch:
          when `!config.is_compose()` and `config.image.is_some()` and
          `config.dockerfile().is_none()` → use image mode
    - [ ] Introduce `BuildSource` enum in `src/provider/docker.rs`:
          `Dockerfile(String)` | `Image(String)`. Add `build_source: BuildSource`
          field to `Docker` struct. Update `build()` to dispatch on `build_source`:
          Dockerfile → `docker build -f ...` as now; Image → `docker pull <image>`
    - [ ] Do the same for `Podman` struct and `Podman::build()`
    - [ ] Update `Docker::create()` and `Podman::create()` to use the image name
          from `BuildSource::Image` when appropriate (instead of `self.file` which
          would be the Dockerfile path)
    - [ ] Add test fixture `tests/fixtures/devcontainer_image.json` with
          `"name": "image-test"` and `"image": "alpine"`
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(config): add image field support for non-Dockerfile devcontainers`
- [x] Task: Write unit tests for image mode
    - [x] Add unit test in `src/devcontainers/config.rs` verifying `Config::parse`
          succeeds with `devcontainer_image.json` and `config.image == Some("alpine")`
    - [x] `cargo test` must pass
    - [x] Commit: `feat(config): add image field support for non-Dockerfile devcontainers`
- [x] Task: Conductor - User Manual Verification 'Phase 1: image Field Support' (Protocol in workflow.md)

## Phase 2: Missing Lifecycle Hooks (FR-002 and FR-003)

- [x] Task: Parse `postStartCommand`, `postAttachCommand`, `initializeCommand`
    - [ ] Add to `Config` struct (after FR-004 merges OneOrMany, these will use it;
          for now use `Option<String>` and migrate in Phase 3):
          `post_start_command`, `post_attach_command`, `initialize_command`
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(config): parse postStartCommand, postAttachCommand, initializeCommand`
- [x] Task: Execute hooks in `Devcontainer::run()` (FR-002 and FR-003)
    - [ ] In `Devcontainer::run()`, execute `initializeCommand` on the **host** before
          `self.create(use_cache)` using `std::process::Command::new("sh").arg("-c")`
    - [ ] After `provider.start()`, execute `postStartCommand` inside the container
          via `provider.exec()`
    - [ ] After `provider.attach()`, execute `postAttachCommand` inside the container
          via `provider.exec()`
    - [ ] `cargo test` must pass
    - [ ] Commit: `feat(devcontainer): execute postStartCommand, postAttachCommand, initializeCommand`
- [x] Task: Conductor - User Manual Verification 'Phase 2: Missing Lifecycle Hooks' (Protocol in workflow.md)

## Phase 3: `OneOrMany` Hook Type (FR-004)

- [x] Task: Implement `OneOrMany` serde type
    - [ ] Create `src/one_or_many.rs` (or add to `src/devcontainers/config.rs`)
          with a `OneOrMany` enum: `One(String)` | `Many(Vec<String>)`
    - [ ] Implement `serde::Deserialize` for `OneOrMany` to accept both
          `"string"` and `["arr", "ay"]` JSON forms
    - [ ] Implement a helper `fn to_exec_parts(&self) -> (&str, Vec<&str>)` that
          returns `("sh", vec!["-c", cmd])` for `One(cmd)` and
          `(parts[0], parts[1..])` for `Many(parts)`
    - [ ] Add unit tests for both JSON forms
    - [ ] `cargo test` must pass
    - [ ] Commit: `feat(config): add OneOrMany serde type for lifecycle hooks`
- [x] Task: Migrate all six lifecycle hooks to `OneOrMany`
    - [ ] Change `on_create_command`, `update_content_command`, `post_create_command`,
          `post_start_command`, `post_attach_command`, `initialize_command` in
          `Config` from `Option<String>` to `Option<OneOrMany>`
    - [ ] Update `post_create()` and `run()` in `src/devcontainers/mod.rs` to use
          `to_exec_parts()` for each hook
    - [ ] For `initializeCommand` (host-side), use the first part as program and
          remainder as args directly in `std::process::Command`
    - [ ] Update existing test fixtures if needed
    - [ ] `cargo test` must pass
    - [ ] Commit: `refactor(config): migrate all lifecycle hooks to OneOrMany type`
- [x] Task: Conductor - User Manual Verification 'Phase 3: OneOrMany Hook Type' (Protocol in workflow.md)

## Phase 4: Quality Gate

- [x] Task: Run full quality gate
    - [x] `cargo test` ✓
    - [x] `cargo test --test integration` ✓
    - [x] `cargo clippy --all-targets -- -D warnings` ✓
    - [x] `cargo fmt --check` ✓
    - [x] `cargo deny check licenses bans advisories` ✓
    - [x] Verify no new `#[allow(...)]` without justification
- [x] Task: Conductor - User Manual Verification 'Phase 4: Quality Gate' (Protocol in workflow.md)
