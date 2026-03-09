# Spec: Provider exec_raw — Injection-Safe Direct Exec for Many Hooks

## Problem

`OneOrMany::Many(parts)` was designed so that lifecycle hooks specified as an array execute
directly (no shell), making them injection-safe for arguments containing spaces. However the
current `Provider::exec(&self, cmd: String)` signature only accepts a shell string, so
`exec_hook` must join parts with spaces before passing to exec, where the provider then wraps
them in `sh -c`. This defeats the design intent:

```rust
// exec_hook today — Many is re-shelled, not injection-safe
OneOrMany::Many(parts) => provider.exec(parts.join(" ")),
//                                       ^^ space-join → sh -c, injection risk
```

Example: `["rm", "-rf", "/important path"]` becomes `sh -c "rm -rf /important path"` which
splits `/important path` incorrectly.

## Goal

Add `fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool>` to the `Provider` trait,
implement it for all four providers (Docker, Podman, DockerCompose, PodmanCompose), and update
`exec_hook` to use it for the `Many` variant.

## Functional Requirements

- FR-001: `Provider` trait gains `exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool>`
- FR-002: `exec_raw` executes without a shell wrapper (no `sh -c`)
- FR-003: All four providers implement `exec_raw`
- FR-004: `exec_hook` uses `exec_raw` (via `to_exec_parts()`) for the `Many` variant
- FR-005: `exec_hook` One variant continues to use `exec` (shell-wrapped as before)

## Out of Scope

- Changing `exec` signature
- Changing how `exec_host_hook` works (already uses `to_exec_parts()` correctly)
