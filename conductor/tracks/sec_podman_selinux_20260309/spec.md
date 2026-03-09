# Spec: Security — Add SELinux Label Support for Podman SSH Socket Mount

## Problem

When Podman runs on a host with SELinux enforcing (RHEL, Fedora, CentOS), mounting the SSH agent
socket into the container requires the `:z` or `:Z` relabelling suffix on the volume mount
(e.g., `$SSH_AUTH_SOCK:/ssh-agent:z`). Without it, the container process gets a permission denied
error when trying to use the socket, even though the mount itself succeeds.

The current compose override template does not include the `:z` suffix, making the SSH agent
non-functional on SELinux systems.

## Goal

Add optional `:z` relabelling to the SSH agent socket mount in the Podman Compose override,
controlled by a detection step or a config option.

## Functional Requirements

- FR-001: Detect at runtime whether SELinux is enforcing: check if `/sys/fs/selinux/enforce`
  reads `"1"`.
- FR-002: If SELinux is enforcing and the provider is Podman or PodmanCompose, append `:z` to
  the SSH socket volume entry in the compose override template.
- FR-003: Expose an optional `"selinuxRelabel": true/false` field in `devcontainer.json`
  (parsed into `Config`) that overrides auto-detection.
- FR-004: Unit tests: detection logic, template output with `:z` suffix, template without suffix
  when SELinux is not enforcing.
- FR-005: `cargo test` and `cargo clippy` pass.

## Out of Scope

- Applying `:z` to user-defined mounts in `mounts` (future work, separate track).
