# Spec: build_source_for Refactor — Replace bool Parameter with Explicit Enum

## Problem

`build_source_for` in `src/devcontainers/mod.rs` uses a positional `in_devcontainer: bool`
parameter to distinguish whether to prefix the Dockerfile path with `.devcontainer/`:

```rust
fn build_source_for(directory: &Path, config: &Config, in_devcontainer: bool) -> std::io::Result<BuildSource>
```

The name is misleading — it doesn't mean "we are inside a devcontainer", it means "the
Dockerfile lives inside a `.devcontainer/` subdirectory". Call sites (`false` for Docker,
`true` for Podman) are not self-documenting.

## Goal

Replace the `bool` with a local enum or two separate helpers so call sites read clearly.

## Functional Requirements

- FR-001: The `in_devcontainer: bool` parameter is replaced with something self-documenting
- FR-002: Behaviour is unchanged (Docker: `directory/dockerfile`, Podman: `directory/.devcontainer/dockerfile`)
- FR-003: All call sites updated, `cargo clippy` passes

## Options

**Option A** — Two dedicated helpers:
```rust
fn docker_build_source(directory, config) -> Result<BuildSource>
fn podman_build_source(directory, config) -> Result<BuildSource>
```

**Option B** — Local enum parameter:
```rust
enum DockerfileLookup { Direct, InDevcontainerDir }
fn build_source_for(directory, config, lookup: DockerfileLookup) -> Result<BuildSource>
```

Preferred: Option A (simpler, more obvious).
