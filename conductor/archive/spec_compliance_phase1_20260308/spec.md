# Spec: Dev Containers Spec Compliance — Phase 1

## Overview

Address the highest-impact gaps between `devcont`'s current behaviour and the
Dev Containers specification. Phase 1 targets four areas where the current
implementation blocks real-world devcontainers or silently produces wrong results.

**Depends on:** none (standalone)

---

## Pre-identified Gaps

### Gap 1 — `image` field not supported
The entire non-compose code path requires `build.dockerfile`. Devcontainers
that specify a pre-built image (`"image": "mcr.microsoft.com/devcontainers/rust"`)
fail immediately with a "missing required field" error. This blocks the majority
of real-world devcontainer.json files.

### Gap 2 — `postStartCommand` and `postAttachCommand` not implemented
These two lifecycle hooks are not parsed from `devcontainer.json` and never
executed. `postStartCommand` runs on every container start; `postAttachCommand`
runs after each client attach. Both are widely used in practice.

### Gap 3 — `initializeCommand` not implemented
Host-side pre-flight logic that runs before the container is created. Not parsed
and not executed.

### Gap 4 — Lifecycle hooks only accept strings, not arrays
The spec allows hook values as either a string (`"npm install"`) or an array
(`["npm", "install"]`). All hooks are typed as `Option<String>`; an array value
causes a parse error and prevents the container from starting.

---

## Functional Requirements

### FR-001: `image` field support
Parse `"image": String` in `Config`. In `build_provider`, when `image` is set
and the container is not compose-based:
- `build()` → run `docker/podman pull <image>` to ensure the image is present
- `create()` → `docker/podman create` using `<image>` as the image reference

Add `image: Option<String>` to `Config`. Introduce a `BuildSource` enum
(`Dockerfile(String)` | `Image(String)`) used by `Docker` and `Podman` providers
to switch between build modes. When both `image` and `build.dockerfile` are
present, `build.dockerfile` takes precedence (spec §build priority).

Add a test fixture `tests/fixtures/devcontainer_image.json` with
`"image": "alpine"` and no `build` block.

### FR-002: `postStartCommand` and `postAttachCommand` hooks
Parse `post_start_command: Option<OneOrMany>` and
`post_attach_command: Option<OneOrMany>` in `Config`
(see FR-004 for `OneOrMany` definition).

Execution points in `Devcontainer::run()`:
- `postStartCommand`: execute inside the container immediately after
  `provider.start()` returns, on every invocation of `run()`.
- `postAttachCommand`: execute inside the container immediately after
  `provider.attach()` returns.

### FR-003: `initializeCommand` hook
Parse `initialize_command: Option<OneOrMany>` in `Config`.
Execute on the **host** (not inside the container) before `provider.build()` is
called, using `std::process::Command`. The working directory for the host command
is the project directory passed to `Devcontainer::load()`.

### FR-004: `OneOrMany` serde type for lifecycle hooks
Introduce a `OneOrMany` type (or equivalent) that deserialises from JSON as
either a plain string or an array of strings:

```json
"postCreateCommand": "npm install"          // string form
"postCreateCommand": ["npm", "install"]     // array form
```

Apply `OneOrMany` to all six lifecycle hook fields:
`onCreateCommand`, `updateContentCommand`, `postCreateCommand`,
`postStartCommand`, `postAttachCommand`, `initializeCommand`.

When executing an array hook, pass the first element as the program and the
remainder as direct arguments (no shell — safe from injection).
When executing a string hook, pass to `sh -c` as before.

---

## Non-Functional Requirements

- All new config fields are optional; existing devcontainer.json files continue
  to work without changes
- Each FR is implemented in its own commit or small commit set
- No regressions in existing unit or integration tests

---

## Acceptance Criteria

- A devcontainer.json with `"image": "alpine"` and no `build` block starts
  successfully with Docker and Podman providers
- `postStartCommand` is executed after every `provider.start()` call
- `postAttachCommand` is executed after `provider.attach()` returns
- `initializeCommand` is executed on the host before `provider.build()`
- All six lifecycle hooks accept both string and `["array", "form"]` values
  from JSON without parse errors
- All quality gates pass

---

## Out of Scope

- `features` (Dev Container Features / OCI extensions)
- `containerEnv` (distinct from `remoteEnv`)
- `customizations` (IDE-specific settings)
- `hostRequirements` (CPU/memory pre-flight checks)
- `forwardPorts` in compose providers
- Multi-config support (`.devcontainer/<folder>/devcontainer.json`)
- `shutdownAction: stopCompose` distinction
