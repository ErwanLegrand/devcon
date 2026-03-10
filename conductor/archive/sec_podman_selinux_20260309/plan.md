# Plan: Security — Add SELinux Label Support for Podman SSH Socket Mount

## Phase 1: SELinux Detection and Template Update

- [ ] Task: Add SELinux enforcement detection
    - [ ] In `src/provider/utils.rs`, add `fn selinux_enforcing() -> bool`
    - [ ] Read `/sys/fs/selinux/enforce`; return `true` if content is `"1"`
    - [ ] Return `false` if file does not exist or read fails
    - [ ] `cargo build` must pass
- [ ] Task: Update compose override template for `:z` suffix
    - [ ] In the compose override template (`src/provider/templates/`), add a conditional
          `{{ if selinux }}:z{{ endif }}` suffix to the SSH socket volume entry
    - [ ] Add `selinux: bool` field to `TemplateContext`
    - [ ] Set `selinux` in `create_compose_override` based on detection result and
          `Config::selinux_relabel` override
    - [ ] Add `selinux_relabel: Option<bool>` to `Config` struct
    - [ ] `cargo build` must pass
    - [ ] Commit: `feat(podman): add SELinux :z relabelling for SSH socket mount`

## Phase 2: Tests

- [ ] Task: Unit tests for SELinux detection and template rendering
    - [ ] `selinux_enforcing()` returns true when file reads `"1"`
    - [ ] `selinux_enforcing()` returns false when file missing
    - [ ] Template with `selinux: true` includes `:z` suffix
    - [ ] Template with `selinux: false` omits `:z`
    - [ ] `config.selinux_relabel = Some(true)` overrides detection
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(podman): SELinux relabelling detection and template`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
