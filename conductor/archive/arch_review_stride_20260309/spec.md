# Spec: Architecture Review & STRIDE Threat Modelling

## Objective

Perform a structured review of `devcont`'s architecture and apply the STRIDE threat model to
identify security risks. Produce a report with findings, risk ratings, and recommended
mitigations. The report becomes a reference document for future security work and architecture
decisions.

## Scope

### Architecture Review

Analyse the following architectural concerns:

1. **Module structure** — Is the boundary between `devcontainers/` (orchestration) and
   `provider/` (engine adapters) well-defined? Are responsibilities clearly separated?
2. **Provider trait design** — Does the `Provider` trait surface the right abstraction level?
   Are there leaky abstractions or operations that should be composed differently?
3. **Config parsing pipeline** — `devcontainer.json` → `Config` → `build_provider` →
   `Devcontainer`. Is the data flow correct? Are there parse/validate gaps?
4. **Error propagation** — Is `std::io::Error` the right error type throughout? Are error
   messages actionable? Are errors swallowed anywhere?
5. **Lifecycle hook execution** — `initializeCommand` (host), `onCreateCommand`,
   `updateContentCommand`, `postCreateCommand`, `postStartCommand`, `postAttachCommand`
   (container). Are ordering guarantees correct? Are failures handled?
6. **Settings & configuration layering** — How are user settings (`~/.config/devcont/`) and
   project settings (`devcontainer.json`) combined? Is there a priority order?

### STRIDE Threat Model

For each STRIDE category, enumerate threats against the `devcont` attack surface:

| Category | Description |
|---|---|
| **S** Spoofing | Can an attacker impersonate a trusted identity? |
| **T** Tampering | Can data or code be modified without detection? |
| **R** Repudiation | Can actions be denied or go unlogged? |
| **I** Information Disclosure | Can sensitive data leak? |
| **D** Denial of Service | Can the tool or container be made unavailable? |
| **E** Elevation of Privilege | Can an attacker gain higher privileges? |

Key attack surfaces to model:
- `devcontainer.json` as an untrusted input (workspace-supplied config)
- Compose override file written to `/tmp` (world-readable temp dir)
- SSH agent socket forwarding (`SSH_AUTH_SOCK`)
- Host-side `initializeCommand` execution
- Container exec commands constructed from config values
- Docker/Podman socket access (DooD)
- Image names and Dockerfile paths sourced from config

## Deliverable

A markdown report saved to `conductor/tracks/arch_review_stride_20260309/report.md` containing:

1. **Architecture Findings** — each finding with: area, severity (Critical/High/Medium/Low/Info),
   description, and recommendation.
2. **STRIDE Threat Table** — one row per threat: category, threat description, affected
   component, likelihood (High/Med/Low), impact (High/Med/Low), and proposed mitigation.
3. **Prioritised Action List** — ordered list of mitigations to implement, tagged with
   suggested track IDs for follow-up.

## Output Artefact

`conductor/tracks/arch_review_stride_20260309/report.md`
